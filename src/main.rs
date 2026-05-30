use clap::{Parser as ClapParser, Subcommand};
use miette::Result;
use nimble::Parser;
use std::path::{Path, PathBuf};

#[derive(ClapParser)]
#[command(
    name = "nimble",
    version,
    about = "Nimble Toolchain – compiler, package manager, and development tools"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new Nimble project.
    Init {
        path: PathBuf,
        #[arg(long)]
        name: Option<String>,
    },

    /// Build the current project using the manifest.
    Build {
        #[arg(default_value = ".")]
        path: PathBuf,
        #[arg(short = 'r', long)]
        run: bool,
        #[arg(short = 'c', long)]
        clean: bool,
    },

    /// Run the current project.
    Run {
        #[arg(default_value = ".")]
        path: PathBuf,
    },

    /// Compile a single Nimble source file.
    Compile {
        file: String,
        #[arg(short = 'o', long)]
        output: Option<String>,
        #[arg(long)]
        emit_llvm: bool,
        #[arg(short = 'r', long)]
        run: bool,
        #[arg(short = 'c', long)]
        clean: bool,
    },

    /// Format Nimble source code.
    Fmt { file: PathBuf },

    /// Start the Nimble REPL.
    Repl,

    /// Start the Nimble LSP server.
    Lsp,

    /// Install a standalone executable binary.
    Install {
        #[arg(value_name = "URI@VERSION")]
        target: String,
    },

    /// Uninstall a previously installed binary.
    Uninstall {
        #[arg(value_name = "NAME")]
        name: String,
    },

    /// Upgrade an installed binary.
    Upgrade {
        #[arg(value_name = "URI@VERSION")]
        target: String,
    },

    /// Library package management.
    Pkg {
        #[command(subcommand)]
        action: PkgAction,
    },

    /// Fetch dependencies for the local project.
    Fetch {
        #[arg(default_value = ".")]
        path: PathBuf,
    },

    /// Generate documentation for a Nimble project
    Doc {
        #[arg(default_value = ".")]
        path: PathBuf,
        #[arg(long, default_value = "docs")]
        output_dir: String,
    },

    /// Profile a Nimble program
    Profile {
        file: String,
        #[arg(short = 'o', long)]
        output: Option<String>,
    },

    /// Fuzz the compiler to find crashes
    Fuzz {
        #[arg(long, default_value = "1000")]
        iterations: u64,
        #[arg(long, default_value = "42")]
        seed: u64,
    },

    /// Generate the runtime C header for self-hosting
    GenerateHeader {
        #[arg(default_value = "nimble_runtime.h")]
        output: String,
    },

    /// Lint a Nimble source file
    Lint { file: String },
}

#[derive(Subcommand)]
enum PkgAction {
    /// Cache a library package globally.
    Install {
        #[arg(value_name = "URI@VERSION")]
        target: String,
    },
    /// Remove a cached library package.
    Uninstall {
        #[arg(value_name = "URI@VERSION")]
        target: String,
    },
    /// Re-clone a cached library package.
    Upgrade {
        #[arg(value_name = "URI@VERSION")]
        target: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { path, name } => {
            let project_name = name.unwrap_or_else(|| {
                path.file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string()
            });
            nimble::anvil::commands::init_project(&path, &project_name)
                .map_err(|e| miette::miette!(e))?;
        }
        Commands::Build { path, run, clean } => {
            nimble::anvil::commands::build_project(&path, run, clean)
                .map_err(|e| miette::miette!(e))?;
        }
        Commands::Run { path } => {
            nimble::anvil::commands::run_project(&path).map_err(|e| miette::miette!(e))?;
        }
        Commands::Compile {
            file,
            output,
            emit_llvm,
            run,
            clean,
        } => {
            let source = std::fs::read_to_string(&file)
                .map_err(|e| miette::miette!("cannot read `{}`: {}", file, e))?;

            let out_path = output.unwrap_or_else(|| {
                let p = Path::new(&file).with_extension("exe");
                p.to_string_lossy().to_string()
            });

            let opts = nimble::smelt::driver::CompileOptions {
                output_path: out_path.clone(),
                source_path: Some(file.clone()),
                emit_llvm,
                run_after: run,
                ..Default::default()
            };

            nimble::smelt::driver::compile(&source, &opts).map_err(|e| miette::miette!(e))?;

            if clean && run {
                let _ = std::fs::remove_file(&out_path);
            }
        }
        Commands::Fmt { file } => {
            let source = std::fs::read_to_string(&file)
                .map_err(|e| miette::miette!("cannot read {}: {}", file.display(), e))?;

            let prog = match Parser::new(&source) {
                Ok(mut p) => p
                    .parse()
                    .map_err(|e| miette::miette!("parse error: {}", e))?,
                Err(e) => return Err(miette::miette!("lex error: {}", e)),
            };

            let formatted = nimble::chisel::fmt::format_program(&prog);
            std::fs::write(&file, formatted)
                .map_err(|e| miette::miette!("failed to write {}: {}", file.display(), e))?;
            eprintln!("formatted {}", file.display());
        }
        Commands::Repl => {
            #[cfg(feature = "jit")]
            {
                nimble::forge::repl_jit::run_repl().map_err(|e| miette::miette!(e))?;
            }
            #[cfg(not(feature = "jit"))]
            {
                eprintln!(
                    "Nimble REPL (IR preview mode - compile with `--features jit` for JIT execution)"
                );
                nimble::forge::repl_simple::run_repl().map_err(|e| miette::miette!(e))?;
            }
        }
        Commands::Lsp => {
            let stdin = tokio::io::stdin();
            let stdout = tokio::io::stdout();
            let (service, socket) =
                tower_lsp::LspService::build(nimble::lantern::lsp::Backend::new).finish();
            tower_lsp::Server::new(stdin, stdout, socket)
                .serve(service)
                .await;
        }
        Commands::Install { target } => {
            let (uri, version) = split_target(&target)?;
            nimble::nim::manager::PackageManager::new()
                .map_err(|e| miette::miette!(e))?
                .install_standalone_binary(uri, version)
                .map_err(|e| miette::miette!(e))?;
        }
        Commands::Uninstall { name } => {
            nimble::nim::manager::PackageManager::new()
                .map_err(|e| miette::miette!(e))?
                .uninstall_binary(&name)
                .map_err(|e| miette::miette!(e))?;
        }
        Commands::Upgrade { target } => {
            let (uri, version) = split_target(&target)?;
            nimble::nim::manager::PackageManager::new()
                .map_err(|e| miette::miette!(e))?
                .upgrade_binary(uri, version)
                .map_err(|e| miette::miette!(e))?;
        }
        Commands::Pkg { action } => {
            let pm = nimble::nim::manager::PackageManager::new().map_err(|e| miette::miette!(e))?;
            match action {
                PkgAction::Install { target } => {
                    let (uri, version) = split_target(&target)?;
                    pm.install_pkg_library(uri, version)
                        .map_err(|e| miette::miette!(e))?;
                }
                PkgAction::Uninstall { target } => {
                    let (uri, version) = split_target(&target)?;
                    pm.uninstall_pkg_library(uri, version)
                        .map_err(|e| miette::miette!(e))?;
                }
                PkgAction::Upgrade { target } => {
                    let (uri, version) = split_target(&target)?;
                    pm.upgrade_pkg_library(uri, version)
                        .map_err(|e| miette::miette!(e))?;
                }
            }
        }
        Commands::Fetch { path } => {
            let manifest = nimble::nim::manager::ProjectManifest::load(&path)
                .map_err(|e| miette::miette!(e))?;
            let pm = nimble::nim::manager::PackageManager::new().map_err(|e| miette::miette!(e))?;
            let cached = pm
                .fetch_manifest_deps(&manifest)
                .map_err(|e| miette::miette!(e))?;
            if cached.is_empty() {
                println!("    Finished no dependencies declared");
            } else {
                println!(
                    "    \x1b[1mFinished\x1b[0m {} package(s) ready",
                    cached.len()
                );
            }
        }
        Commands::Doc { path, output_dir } => {
            let mut docgen = nimble::docgen::DocGenerator::new();
            for entry in std::fs::read_dir(&path).map_err(|e| miette::miette!(e))? {
                let entry = entry.map_err(|e| miette::miette!(e))?;
                let entry_path = entry.path();
                if entry_path.extension().is_some_and(|ext| ext == "nbl") {
                    let source =
                        std::fs::read_to_string(&entry_path).map_err(|e| miette::miette!(e))?;
                    if let Ok(mut p) = nimble::Parser::new(&source)
                        && let Ok(prog) = p.parse()
                    {
                        let module_name = entry_path
                            .file_stem()
                            .map(|s| s.to_string_lossy().to_string())
                            .unwrap_or_else(|| "unknown".to_string());
                        docgen.extract_from_program(&prog, &module_name);
                    }
                }
            }
            std::fs::create_dir_all(&output_dir).map_err(|e| miette::miette!(e))?;
            let html = docgen.to_html();
            let index_path = Path::new(&output_dir).join("index.html");
            std::fs::write(&index_path, html)
                .map_err(|e| miette::miette!("failed to write {}: {}", index_path.display(), e))?;
            eprintln!("docs generated in {}", output_dir);
        }
        Commands::Profile { file, output } => {
            let source = std::fs::read_to_string(&file)
                .map_err(|e| miette::miette!("cannot read `{}`: {}", file, e))?;
            let out_path = output.unwrap_or_else(|| {
                Path::new(&file)
                    .with_extension("exe")
                    .to_string_lossy()
                    .to_string()
            });
            let mut profiler = nimble::profiler::Profiler::new();
            profiler.start("compile");
            let opts = nimble::smelt::driver::CompileOptions {
                output_path: out_path.clone(),
                source_path: Some(file.clone()),
                run_after: true,
                ..Default::default()
            };
            nimble::smelt::driver::compile(&source, &opts).map_err(|e| miette::miette!(e))?;
            profiler.end("compile");
            profiler.write_report();
        }
        Commands::Fuzz { iterations, seed } => {
            let fuzzer = nimble::fuzzer::Fuzzer::new(seed, iterations);
            let crashes = fuzzer.run().map_err(|e| miette::miette!(e))?;
            if crashes.is_empty() {
                eprintln!("fuzzing completed: {} iterations, no crashes", iterations);
            } else {
                for crash in &crashes {
                    eprintln!("{}", crash);
                }
                eprintln!("fuzzing completed: {} crashes found", crashes.len());
            }
        }
        Commands::GenerateHeader { output } => {
            let header = nimble::selfhost::generate_runtime_header();
            std::fs::write(&output, header)
                .map_err(|e| miette::miette!("failed to write {}: {}", output, e))?;
            eprintln!("generated runtime header: {}", output);
        }
        Commands::Lint { file } => {
            let source = std::fs::read_to_string(&file)
                .map_err(|e| miette::miette!("cannot read `{}`: {}", file, e))?;
            let prog = match nimble::Parser::new(&source) {
                Ok(mut p) => p
                    .parse()
                    .map_err(|e| miette::miette!("parse error: {}", e))?,
                Err(e) => return Err(miette::miette!("lex error: {}", e)),
            };
            let mut linter = nimble::lint::Linter::new();
            let warnings = linter.lint_program(&prog);
            if warnings.is_empty() {
                eprintln!("no warnings");
            } else {
                for w in &warnings {
                    eprintln!("warning:{}:{}: {}", w.line, w.column, w.message);
                }
            }
        }
    }

    Ok(())
}

fn split_target(target: &str) -> Result<(&str, &str)> {
    target.rsplit_once('@').ok_or_else(|| {
        miette::miette!("missing version tag in `{}` (expected URI@version)", target)
    })
}
