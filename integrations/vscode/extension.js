const { execFile } = require("child_process");
const vscode = require("vscode");

const activate = (_) => {
  const outputChannel = vscode.window.createOutputChannel("Alejandra");

  vscode.languages.registerDocumentFormattingEditProvider("nix", {
    provideDocumentFormattingEdits(document, _) {
      const config = {
        alejandra: vscode.workspace.getConfiguration("alejandra"),
      };

      return new Promise((resolve, reject) => {
        try {
          outputChannel.appendLine(
            `Running Alejandra with settings: ${JSON.stringify(config)}`
          );

          const process = execFile(
            config.alejandra.program,
            [],
            {},
            (error, stdout, stderr) => {
              if (error) {
                outputChannel.appendLine(`error: ${error}`);
                outputChannel.appendLine(`stderr: ${stderr}`);
                vscode.window.showErrorMessage(
                  `While executing Alejandra with settings: ` +
                    `${JSON.stringify(config)}, ` +
                    `${error}`
                );
                reject(error);
              }

              const documentRange = new vscode.Range(
                document.lineAt(0).range.start,
                document.lineAt(
                  document.lineCount - 1
                ).rangeIncludingLineBreak.end
              );

              resolve([new vscode.TextEdit(documentRange, stdout)]);
            }
          );

          const documentText = document.getText();

          outputChannel.appendLine(
            `Feeding ${documentText.length} of input to stdin`
          );

          process.stdin.write(documentText);
          process.stdin.end();
        } catch (error) {
          vscode.window.showErrorMessage(
            `While executing Alejandra with settings: ` +
              `${JSON.stringify(config)} ` +
              `${error}`
          );
          reject(error);
        }
      });
    },
  });
};

const deactivate = () => {};

module.exports = {
  activate,
  deactivate,
};
