// When the user presses F1, it should return to the main menu.
export async function registerMenuKeybind() {
  document.addEventListener('keydown', async (e) => {
    // @ts-expect-error womp womp
    const isDev = await __TAURI_INTERNALS__.invoke('is_dev')

    console.log(e.key)

    if (e.key === 'F1') {
      window.location.href = isDev ? 'http://localhost:1420' : 'http://tauri.localhost'
    }
  })
}