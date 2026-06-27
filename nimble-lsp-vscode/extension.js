const vscode = require("vscode");
const path = require("path");
const fs = require("fs");
const cp = require("child_process");
const { LanguageClient } = require("vscode-languageclient/node");

let client;

function activate(context) {
  console.log("[nimble-lsp] activating extension");
  vscode.window.showInformationMessage("Nimble LSP activating...");

  const langId = "nimble";
  const ext = ".nbl";

  // Force .nbl files to nimble language
  context.subscriptions.push(
    vscode.workspace.onDidOpenTextDocument((doc) => {
      if (doc.fileName.endsWith(ext) && doc.languageId !== langId) {
        vscode.languages.setTextDocumentLanguage(doc, langId).catch(() => {});
      }
    })
  );

  // Also handle already-open documents
  vscode.workspace.textDocuments.forEach((doc) => {
    if (doc.fileName.endsWith(ext) && doc.languageId !== langId) {
      vscode.languages.setTextDocumentLanguage(doc, langId).catch(() => {});
    }
  });

  // Find nimble binary
  const config = vscode.workspace.getConfiguration("nimble");
  let binary = config.get("lsp.command");

  const ws = vscode.workspace.workspaceFolders?.[0]?.uri?.fsPath;
  const isWindows = process.platform === "win32";

  if (!binary) {
    const candidates = [
      ws && path.join(ws, "target/release/nimble"),
      ws && path.join(ws, "target/debug/nimble"),
    ].filter(Boolean);

    binary = "nimble";
    for (const c of candidates) {
      const p = isWindows ? `${c}.exe` : c;
      if (fs.existsSync(p)) {
        binary = p;
        break;
      }
    }
  } else {
    // Resolve ${workspaceFolder} if present
    if (binary.includes("${workspaceFolder}")) {
      if (ws) {
        binary = binary.replace(/\$\{workspaceFolder\}/g, ws);
      }
    }
    // Fallback to .exe on Windows if needed
    if (isWindows && !binary.endsWith(".exe")) {
      if (fs.existsSync(binary + ".exe")) {
        binary = binary + ".exe";
      }
    }
  }

  console.log("[nimble-lsp] binary:", binary);

  // Register formatting
  context.subscriptions.push(
    vscode.languages.registerDocumentFormattingEditProvider(langId, {
      provideDocumentFormattingEdits(document) {
        try {
          const p = cp.spawnSync(binary, ["fmt", "--stdin"], {
            input: document.getText(),
            encoding: "utf-8",
            timeout: 5000,
          });
          if (p.status === 0 && p.stdout) {
            const lastLine = document.lineAt(document.lineCount - 1);
            const range = new vscode.Range(0, 0, document.lineCount - 1, lastLine.text.length);
            return [vscode.TextEdit.replace(range, p.stdout)];
          }
        } catch (_) {}
        return [];
      },
    })
  );

  // Server options
  const serverOptions = {
    run: { command: binary, args: ["lsp"] },
    debug: { command: binary, args: ["lsp"] },
  };

  // Client options
  const clientOptions = {
    documentSelector: [{ scheme: "file", language: langId }],
  };

  // Create the language client and start the client.
  client = new LanguageClient(
    "nimbleLSP",
    "Nimble Language Server",
    serverOptions,
    clientOptions
  );

  // Start the client. This will also launch the server
  client.start();
  console.log("[nimble-lsp] LanguageClient started");

  context.subscriptions.push({
    dispose: () => {
      if (client) {
        return client.stop();
      }
    }
  });

  console.log("[nimble-lsp] extension activated");
}

function deactivate() {
  if (client) {
    return client.stop();
  }
}

module.exports = { activate, deactivate };
