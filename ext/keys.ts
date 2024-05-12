// When the user presses F1, it should return to the main menu.
export async function registerMenuKeybind() {
  document.addEventListener('keydown', async (e) => {
    if (e.key === 'F1') {
      // @ts-expect-error womp womp
      const isDev = await __TAURI_INTERNALS__.invoke('is_dev')
  
      window.location.href = isDev ? 'http://localhost:1420' : 'http://tauri.localhost'
    }

    // F for fullscreen
    if (e.key === 'F') {
      // @ts-expect-error womp womp
      await __TAURI_INTERNALS__.invoke('toggle_fullscreen')
    }
  })
}