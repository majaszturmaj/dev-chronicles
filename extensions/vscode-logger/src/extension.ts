import * as vscode from "vscode";

async function postEvent(endpoint: string, payload: Record<string, unknown>) {
  const url = `${endpoint.replace(/\/$/, "")}/ingest`;

  try {
    await fetch(url, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        source: "vscode",
        payload,
      }),
    });
  } catch (error) {
    const message =
      error instanceof Error ? error.message : "Unknown error";
    vscode.window.showErrorMessage(
      `DevChronicle: failed to send event (${message}).`
    );
  }
}

export function activate(context: vscode.ExtensionContext) {
  const configuration = vscode.workspace.getConfiguration("devChronicle");
  let endpoint = configuration.get<string>("endpoint", "http://localhost:3030");

  context.subscriptions.push(
    vscode.workspace.onDidSaveTextDocument(async (document) => {
      if (!endpoint) {
        return;
      }

      const workspaceFolder = vscode.workspace.getWorkspaceFolder(document.uri);

      await postEvent(endpoint, {
        event: "save",
        path: document.uri.fsPath,
        languageId: document.languageId,
        workspaceFolder: workspaceFolder?.uri.fsPath,
        timestamp: new Date().toISOString(),
      });
    })
  );

  context.subscriptions.push(
    vscode.workspace.onDidChangeConfiguration((event) => {
      if (event.affectsConfiguration("devChronicle.endpoint")) {
        endpoint = vscode.workspace
          .getConfiguration("devChronicle")
          .get<string>("endpoint", "http://localhost:3030");
      }
    })
  );
}

export function deactivate() {
  // No clean up needed for this lightweight extension.
}

