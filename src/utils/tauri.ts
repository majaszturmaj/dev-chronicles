type InvokeArgs = Record<string, unknown> | undefined;

export function isTauri(): boolean {
  if (typeof window === "undefined") {
    return false;
  }

  const globalWithTauri = window as typeof window & {
    __TAURI__?: Record<string, unknown>;
    __TAURI_INTERNALS__?: Record<string, unknown>;
  };

  return Boolean(globalWithTauri.__TAURI__ || globalWithTauri.__TAURI_INTERNALS__);
}

export async function invokeCommand<T>(command: string, args?: InvokeArgs): Promise<T> {
  if (!isTauri()) {
    throw new Error("Tauri APIs are unavailable. Make sure the app is running inside the Tauri shell.");
  }

  const { invoke } = await import("@tauri-apps/api/core");
  return invoke<T>(command, args);
}

