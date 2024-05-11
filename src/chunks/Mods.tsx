import { useState, useEffect } from 'preact/hooks'

import './Mods.css'
import { invoke } from '@tauri-apps/api/core'

export function Mods() {
  const [mods, setMods] = useState<string[]>([])

  useEffect(() => {
    (async () => {
      const mods: string[] = await invoke('get_replacer_list')
      setMods(mods)
    })()
  }, [])

  return (
    <div class="card">
      <div class="mods">
        <h1>Mods</h1>

        {
          mods.length > 0 ? (
            <div class="mod-list">
              {mods.map((mod) => (
                <div class="mod-item">{mod}</div>
              ))}
            </div>
          ) : (
            <p>No mods installed!</p>
          )
        }
      </div>

      <div
        class="button"
        onClick={() => {
          invoke('open_mods_folder')
        }}
      >
        <button>Open Mods Folder</button>
      </div>
    </div>
  )
}
