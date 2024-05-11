import { registerMenuKeybind } from './keys'
import { createNotifSection, showNotif } from './notifs'
import { startRPCTracker } from './rpc'
import { waitForElement } from './utils'

async function init() {
  console.log('RogueTop injected successfully!')
  
  proxyFetch()

  // If we are not actually in pokerogue, but in the main menu, don't do anything
  if (await waitForElement('#root', 1000).catch(() => null) !== null) {
    console.log('Not in pokerogue, don\'t mind me!')
    return
  }

  // Register binds
  registerMenuKeybind()

  console.log('Fetch proxied successfully!')

  // Inject the notification section
  if (document.querySelector('.notif-section') === null) {
    createNotifSection()
  }

  console.log('Notif section created successfully!')

  await waitForElement('.notif-section')

  showNotif('Press F1 to return to the RogueTop menu', 3000)

  startRPCTracker()
}

function proxyFetch() {
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