import * as path from 'path';
import * as process from 'process';
import { workspace, ExtensionContext } from 'vscode';

import {
  LanguageClient,
  LanguageClientOptions,
  ServerOptions,
} from 'vscode-languageclient/node';

let client: LanguageClient;

export function activate(context: ExtensionContext) {
  let baseName: string;
  const plat = `${process.platform}-${process.arch}`;
  if (plat == "darwin-arm64") {
    baseName = "earthlyls-macos-arm64";
  } else if (plat == "darwin-x64") {
    baseName = "earthlyls-macos-amd64";
  } else if (plat == "linux-x64") {
    baseName = "earthlyls-linux-amd64";
  } else if (plat == "win32-x64") {
    baseName = "earthlyls-windows-amd64.exe";
  } else {
    throw new Error(`unsupported platform: ${plat}`);
  }
  const serverOptions: ServerOptions = {
    command: path.join(context.extensionPath, "server", baseName),
  };

  const clientOptions: LanguageClientOptions = {
    documentSelector: [{ scheme: 'file', language: 'earthfile' }],
    synchronize: {
      fileEvents: workspace.createFileSystemWatcher('**/Earthfile')
    }
  };

  client = new LanguageClient(
    'earthlyls',
    'Earthly Language Server',
    serverOptions,
    clientOptions
  );
  client.start();
}

export function deactivate(): Thenable<void> | undefined {
  if (!client) {
    return undefined;
  }
  return client.stop();
}
