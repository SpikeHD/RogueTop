// Tauri runtime-injected global
interface TauriInternals {
  invoke<T = unknown>(cmd: string, args?: Record<string, unknown>): Promise<T>
}

declare const __TAURI_INTERNALS__: TauriInternals

// Custom property added to Window to preserve the original fetch
interface Window {
  nativeFetch: typeof fetch
}
