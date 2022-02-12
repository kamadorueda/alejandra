const vscode = require('vscode');
const { execFileSync } = require("child_process");

function activate(context) {
	const config = vscode.workspace.getConfiguration("alejandra");

  context.subscriptions.push(
    vscode.languages.registerDocumentFormattingEditProvider("nix", {
      provideDocumentFormattingEdits: (document, _, _) => {
        const range = new vscode.Range(0, 0, document.lineCount, 0);

        try {
          const formattedText = execFileSync(config.path, { input: document.getText() });
          return [vscode.TextEdit.replace(range, formattedText.toString())];
        }
        catch (e) {
          vscode.window.showErrorMessage(`alejandra failed: ${e}`);
        }

        return;
      }
    })
  );
}

function deactivate() {}

module.exports = {
	activate,
	deactivate
}
