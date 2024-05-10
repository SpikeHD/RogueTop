async function init() {
  console.log('RogueTop injected successfully!')

  // overwrite fetch to send to the Tauri backend
  // @ts-expect-error womp womp
  window.nativeFetch = window.fetch

  // @ts-expect-error womp womp
  window.fetch = async (url: string, options: RequestInit) => {
    if (!url.includes(':8001')) {
      // Forward to regular fetch
      // @ts-expect-error womp womp
      return window.nativeFetch(url, options)
    }

    console.log(url)
    console.log(options)

    // @ts-expect-error womp womp
    const response = await __TAURI_INTERNALS__.invoke('api_request', {
      url,
      options: JSON.stringify(options)
    })
    
    // Adherence to what most scripts will expect to have available when they are using fetch(). These have to pretend to be promises
    return {
      json: async () => JSON.parse(response),
      text: async () => response,
      arrayBuffer: async () => {
        // Create a new arraybuffer
        const buffer = new ArrayBuffer(response.length)
        const view = new Uint8Array(buffer)

        // Copy the data over
        response.forEach((byte: number, i: number) => view[i] = byte)

        return buffer
      },
      ok: true,
    }
  }
}

init()