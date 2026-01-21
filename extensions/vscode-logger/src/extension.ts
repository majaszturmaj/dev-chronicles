import * as vscode from "vscode";

async function postEvent(endpoint: string, payload: Record<string, unknown>) {
  const url = `${endpoint.replace(/\/$/, "")}/ingest/vscode`;

  try {
    const response = await fetch(url, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        source: "vscode",
        payload,
      }),
    });
    
    if (!response.ok) {
      console.error(`DevChronicle: server returned error ${response.status}`);
    }
  } catch (error) {
    const message =
      error instanceof Error ? error.message : "Unknown error";
    // Only show error in debug mode to avoid annoying users
    console.error(`DevChronicle: failed to send event (${message})`);
  }
}

export function activate(context: vscode.ExtensionContext) {
  const configuration = vscode.workspace.getConfiguration("devChronicle");
  let endpoint = configuration.get<string>("endpoint", "http://localhost:3030");

  const fileOpenTimes = new Map<string, number>();

  // Track file open events
  context.subscriptions.push(
    vscode.workspace.onDidOpenTextDocument(async (document) => {
      if (!endpoint || document.uri.scheme !== "file") {
        return;
      }

      const workspaceFolder = vscode.workspace.getWorkspaceFolder(document.uri);
      fileOpenTimes.set(document.uri.fsPath, Date.now());

      await postEvent(endpoint, {
        event: "file_open",
        file: document.uri.fsPath,
        language: document.languageId,
        workspace: workspaceFolder?.uri.fsPath,
        cursor_line: 1,
        cursor_col: 1,
      });
    })
  );

  // Track file save events
  context.subscriptions.push(
    vscode.workspace.onDidSaveTextDocument(async (document) => {
      if (!endpoint || document.uri.scheme !== "file") {
        return;
      }

      const workspaceFolder = vscode.workspace.getWorkspaceFolder(document.uri);
      const openTime = fileOpenTimes.get(document.uri.fsPath) || Date.now();
      const timeSpent = Math.floor((Date.now() - openTime) / 1000);

      await postEvent(endpoint, {
        event: "file_save",
        file: document.uri.fsPath,
        language: document.languageId,
        workspace: workspaceFolder?.uri.fsPath,
        time_spent_sec: timeSpent,
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

