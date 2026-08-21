const vscode = require('vscode');
const { spawn } = require('child_process');
const path = require('path');
const os = require('os');
const fs = require('fs');

let diagnosticCollection;
let terminal;

/**
 * @param {vscode.ExtensionContext} context
 */
function activate(context) {
    diagnosticCollection = vscode.languages.createDiagnosticCollection('vietlang');
    context.subscriptions.push(diagnosticCollection);

    // 1. Command: Run VietLang File
    let runCommand = vscode.commands.registerCommand('vietlang.run', function () {
        const editor = vscode.window.activeTextEditor;
        if (!editor) {
            vscode.window.showErrorMessage('No active VietLang file open.');
            return;
        }

        const document = editor.document;
        if (document.languageId !== 'vietlang' && !document.fileName.endsWith('.vl')) {
            vscode.window.showWarningMessage('Current active file is not a VietLang (.vl) file.');
            return;
        }

        // Save file first
        document.save().then(() => {
            if (!terminal || terminal.exitStatus !== undefined) {
                terminal = vscode.window.createTerminal({
                    name: 'VietLang Terminal',
                    hideFromUser: false
                });
            }
            terminal.show(true);

            const filePath = document.fileName;
            // Determine executable path
            const vietlangBin = getVietlangBinary();
            terminal.sendText(`${vietlangBin} run "${filePath}"`);
        });
    });

    // 2. Command: Check Syntax & AST
    let checkCommand = vscode.commands.registerCommand('vietlang.check', function () {
        const editor = vscode.window.activeTextEditor;
        if (!editor) return;
        runLinter(editor.document, true);
    });

    // 3. Command: Run Test Suite
    let testCommand = vscode.commands.registerCommand('vietlang.test', function () {
        const editor = vscode.window.activeTextEditor;
        if (!editor) return;

        editor.document.save().then(() => {
            if (!terminal || terminal.exitStatus !== undefined) {
                terminal = vscode.window.createTerminal('VietLang Terminal');
            }
            terminal.show(true);
            const vietlangBin = getVietlangBinary();
            terminal.sendText(`${vietlangBin} test "${editor.document.fileName}"`);
        });
    });

    // 4. Linter & Real-Time Diagnostics on Save / Open / Change
    context.subscriptions.push(
        vscode.workspace.onDidSaveTextDocument((doc) => {
            if (doc.languageId === 'vietlang' || doc.fileName.endsWith('.vl')) {
                runLinter(doc, false);
            }
        }),
        vscode.workspace.onDidOpenTextDocument((doc) => {
            if (doc.languageId === 'vietlang' || doc.fileName.endsWith('.vl')) {
                runLinter(doc, false);
            }
        }),
        vscode.workspace.onDidCloseTextDocument((doc) => {
            diagnosticCollection.delete(doc.uri);
        })
    );

    // Initial check for active editor
    if (vscode.window.activeTextEditor) {
        const doc = vscode.window.activeTextEditor.document;
        if (doc.languageId === 'vietlang' || doc.fileName.endsWith('.vl')) {
            runLinter(doc, false);
        }
    }

    context.subscriptions.push(runCommand, checkCommand, testCommand);
}

function getVietlangBinary() {
    const customPath = vscode.workspace.getConfiguration('vietlang').get('executablePath');
    if (customPath && customPath.trim().length > 0) {
        return customPath;
    }
    const userHome = os.homedir();
    const installedBin = path.join(userHome, '.vietlang', 'bin', 'vietlang');
    if (fs.existsSync(installedBin)) {
        return `"${installedBin}"`;
    }
    return 'vietlang';
}

function runLinter(document, showNotification) {
    if (document.languageId !== 'vietlang' && !document.fileName.endsWith('.vl')) {
        return;
    }

    const filePath = document.fileName;
    let vietlangBin = getVietlangBinary().replace(/"/g, '');

    const child = spawn(vietlangBin, ['check', filePath]);
    let stderr = '';
    let stdout = '';

    child.stdout.on('data', (data) => {
        stdout += data.toString();
    });

    child.stderr.on('data', (data) => {
        stderr += data.toString();
    });

    child.on('close', (code) => {
        const diagnostics = [];
        diagnosticCollection.delete(document.uri);

        if (code !== 0) {
            const output = (stderr + '\n' + stdout).trim();
            // Parse error format: "ParserError at line 10:5: message" or "line 10: message"
            const lines = output.split('\n');
            for (const line of lines) {
                const match = line.match(/(?:at\s+)?line\s+(\d+)(?::(\d+))?:\s*(.*)/i);
                if (match) {
                    const lineNum = Math.max(0, parseInt(match[1], 10) - 1);
                    const colNum = match[2] ? Math.max(0, parseInt(match[2], 10) - 1) : 0;
                    const message = match[3] || line;

                    const range = new vscode.Range(
                        lineNum,
                        colNum,
                        lineNum,
                        colNum + 20
                    );

                    const diagnostic = new vscode.Diagnostic(
                        range,
                        message,
                        vscode.DiagnosticSeverity.Error
                    );
                    diagnostic.source = 'VietLang';
                    diagnostics.push(diagnostic);
                } else if (line.includes('Error') || line.includes('error:')) {
                    const range = new vscode.Range(0, 0, 0, 50);
                    const diagnostic = new vscode.Diagnostic(
                        range,
                        line,
                        vscode.DiagnosticSeverity.Error
                    );
                    diagnostic.source = 'VietLang';
                    diagnostics.push(diagnostic);
                }
            }

            diagnosticCollection.set(document.uri, diagnostics);
            if (showNotification) {
                vscode.window.showErrorMessage(`VietLang Syntax Error: ${output}`);
            }
        } else {
            if (showNotification) {
                vscode.window.showInformationMessage('VietLang: Syntax & AST check passed successfully!');
            }
        }
    });
}

function deactivate() {
    if (diagnosticCollection) {
        diagnosticCollection.clear();
    }
}

module.exports = {
    activate,
    deactivate
};
