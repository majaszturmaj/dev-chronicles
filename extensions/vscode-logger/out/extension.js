"use strict";
var __createBinding = (this && this.__createBinding) || (Object.create ? (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    var desc = Object.getOwnPropertyDescriptor(m, k);
    if (!desc || ("get" in desc ? !m.__esModule : desc.writable || desc.configurable)) {
      desc = { enumerable: true, get: function() { return m[k]; } };
    }
    Object.defineProperty(o, k2, desc);
}) : (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    o[k2] = m[k];
}));
var __setModuleDefault = (this && this.__setModuleDefault) || (Object.create ? (function(o, v) {
    Object.defineProperty(o, "default", { enumerable: true, value: v });
}) : function(o, v) {
    o["default"] = v;
});
var __importStar = (this && this.__importStar) || (function () {
    var ownKeys = function(o) {
        ownKeys = Object.getOwnPropertyNames || function (o) {
            var ar = [];
            for (var k in o) if (Object.prototype.hasOwnProperty.call(o, k)) ar[ar.length] = k;
            return ar;
        };
        return ownKeys(o);
    };
    return function (mod) {
        if (mod && mod.__esModule) return mod;
        var result = {};
        if (mod != null) for (var k = ownKeys(mod), i = 0; i < k.length; i++) if (k[i] !== "default") __createBinding(result, mod, k[i]);
        __setModuleDefault(result, mod);
        return result;
    };
})();
Object.defineProperty(exports, "__esModule", { value: true });
exports.activate = activate;
exports.deactivate = deactivate;
const vscode = __importStar(require("vscode"));
async function postEvent(endpoint, payload) {
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
    }
    catch (error) {
        const message = error instanceof Error ? error.message : "Unknown error";
        // Only show error in debug mode to avoid annoying users
        console.error(`DevChronicle: failed to send event (${message})`);
    }
}
function activate(context) {
    const configuration = vscode.workspace.getConfiguration("devChronicle");
    let endpoint = configuration.get("endpoint", "http://localhost:3030");
    const fileOpenTimes = new Map();
    // Track file open events
    context.subscriptions.push(vscode.workspace.onDidOpenTextDocument(async (document) => {
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
    }));
    // Track file save events
    context.subscriptions.push(vscode.workspace.onDidSaveTextDocument(async (document) => {
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
    }));
    context.subscriptions.push(vscode.workspace.onDidChangeConfiguration((event) => {
        if (event.affectsConfiguration("devChronicle.endpoint")) {
            endpoint = vscode.workspace
                .getConfiguration("devChronicle")
                .get("endpoint", "http://localhost:3030");
        }
    }));
}
function deactivate() {
    // No clean up needed for this lightweight extension.
}
