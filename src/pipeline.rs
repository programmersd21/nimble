// LLVM optimisation pipeline flags.

/// Optimisation levels mapped to `clang` -O flags.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OptLevel {
    None = 0,
    Less = 1,
    Default = 2,
    Aggressive = 3,
}

impl OptLevel {
    pub fn as_opt_flag(&self) -> String {
        format!("-O{}", *self as u8)
    }
}

/// Fine-grained pass configuration for clang.
#[derive(Debug, Clone)]
pub struct PipelineConfig {
    pub opt_level: OptLevel,
    pub vectorize_slp: bool,
    pub vectorize_loop: bool,
    pub gvn: bool,
    pub sroa: bool,
    pub licm: bool,
    pub slsr: bool,
    pub merge_functions: bool,
    pub target_cpu: Option<String>,
    pub target_features: Option<String>,
    pub reloc_model: String,
    pub code_model: String,
}

impl Default for PipelineConfig {
    fn default() -> Self {
        PipelineConfig {
            opt_level: OptLevel::Aggressive,
            vectorize_slp: true,
            vectorize_loop: true,
            gvn: true,
            sroa: true,
            licm: true,
            slsr: true,
            merge_functions: true,
            target_cpu: None,
            target_features: None,
            reloc_model: "pic".into(),
            code_model: "default".into(),
        }
    }
}

impl PipelineConfig {
    pub fn to_clang_args(&self) -> Vec<String> {
        let mut args = Vec::new();

        args.push(self.opt_level.as_opt_flag());

        if self.vectorize_slp {
            args.push("-vectorize-slp".into());
        }
        if self.vectorize_loop {
            args.push("-vectorize-loops".into());
        }
        if let Some(cpu) = &self.target_cpu {
            args.push("-mcpu".into());
            args.push(cpu.clone());
        }
        if let Some(features) = &self.target_features {
            args.push("-mattr".into());
            args.push(features.clone());
        }
        args.push("-relocation-model".into());
        args.push(self.reloc_model.clone());
        args.push("-code-model".into());
        args.push(self.code_model.clone());

        args
    }

    /// Detect host CPU and pass `-mcpu=native` to target the current machine.
    pub fn native_host_args() -> Vec<String> {
        let mut args = vec![];
        args.push("-mcpu".into());
        args.push("native".into());
        args
    }
}
