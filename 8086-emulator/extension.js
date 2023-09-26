// The module 'vscode' contains the VS Code extensibility API
// Import the module and reference it with the alias vscode in your code below
const vscode = require('vscode');
const child_process = require('child_process');
// This method is called when your extension is activated
// Your extension is activated the very first time the command is executed

/**
 * @param {vscode.ExtensionContext} context
 */
function activate(context) {

	// Use the console to output diagnostic information (console.log) and errors (console.error)
	// This line of code will only be executed once when your extension is activated
	console.log('Congratulations, your extension "8086-emulator" is now active!');

	// The command has been defined in the package.json file
	// Now provide the implementation of the command with  registerCommand
	// The commandId parameter must match the command field in package.json
	let disposable = vscode.commands.registerCommand('8086-emulator.helloWorld', function () {
		// The code you place here will be executed every time your command is executed

		// Display a message box to the user
		vscode.window.showInformationMessage('Hello World from 8086 Emulator!');
	});

	let disposable1 = vscode.commands.registerCommand('8086-emulator.runEmulator', function () {
		const pathToEmulator = vscode.extensions.getExtension('avililla.8086-emulator').extensionPath + '/emulator/emu8086.exe';
		vscode.window.showInformationMessage(`Dirección al emulador: ${pathToEmulator}`);
		child_process.exec(`${pathToEmulator} `, (error, stdout, stderr) => {
			console.log(stdout)
			if (error) {
				vscode.window.showErrorMessage(`Error al ejecutar el emulador: ${stderr}`);
			} else {
				vscode.window.showInformationMessage('Ejecución exitosa!');
			}
		});
	});

	let disposable2 = vscode.commands.registerCommand('8086-emulator.compile', function () {
		const editor = vscode.window.activeTextEditor;

		if (editor) {
			const document = editor.document;
			const fileExtension = document.fileName.split('.').pop();
			if (fileExtension === 'asm') {
				const pathToCompiler = vscode.extensions.getExtension('avililla.8086-emulator').extensionPath + '/compiler/FASM.exe';
				vscode.window.showInformationMessage(`Dirección al compilador: ${pathToCompiler}`);
				//Document filename without extension
				const fileName = document.fileName.split('.').slice(0, -1).join('.');
				child_process.exec(`${pathToCompiler} ${document.fileName} ${fileName}.com`, (error, stdout, stderr) => {
					console.log(stdout)
					if (error) {
						vscode.window.showErrorMessage(`Error al compilar: ${stderr}`);
					} else {
						vscode.window.showInformationMessage('Compilación exitosa!');
					}
				});
			} else {
				vscode.window.showInformationMessage(`El archivo abierto tiene la extensión .${fileExtension}`);
			}
		} else {
			vscode.window.showInformationMessage('No hay ningún archivo abierto.');
		}
	});

	let disposable3 = vscode.commands.registerCommand('8086-emulator.compileAndRun', function () {
		const editor = vscode.window.activeTextEditor;

		if (editor) {
			const document = editor.document;
			const fileExtension = document.fileName.split('.').pop();
			if (fileExtension === 'asm') {
				const pathToCompiler = vscode.extensions.getExtension('avililla.8086-emulator').extensionPath + '/compiler/FASM.exe';
				vscode.window.showInformationMessage(`Dirección al compilador: ${pathToCompiler}`);
				//Document filename without extension
				const fileName = document.fileName.split('.').slice(0, -1).join('.');
				child_process.exec(`${pathToCompiler} ${document.fileName} ${fileName}.com`, (error, stdout, stderr) => {
					console.log(stdout)
					if (error) {
						vscode.window.showErrorMessage(`Error al compilar: ${stderr}`);
					} else {
						vscode.window.showInformationMessage('Compilación exitosa!');
						const pathToEmulator = vscode.extensions.getExtension('avililla.8086-emulator').extensionPath + '/emulator/emu8086.exe';
						vscode.window.showInformationMessage(`Dirección al emulador: ${pathToEmulator}`);
						child_process.exec(`${pathToEmulator} ${fileName}.com`, (error, stdout, stderr) => {
							console.log(stdout)
							if (error) {
								vscode.window.showErrorMessage(`Error al ejecutar el emulador: ${stderr}`);
							} else {
								vscode.window.showInformationMessage('Ejecución exitosa!');
							}
						});
					}
				});
			} else {
				vscode.window.showInformationMessage(`El archivo abierto tiene la extensión .${fileExtension}`);
			}
		} else {
			vscode.window.showInformationMessage('No hay ningún archivo abierto.');
		}
	});
	context.subscriptions.push(disposable, disposable1, disposable2,disposable3);
}

// This method is called when your extension is deactivated
function deactivate() {}

module.exports = {
	activate,
	deactivate
}
