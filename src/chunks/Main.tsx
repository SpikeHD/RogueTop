import { useEffect, useState } from 'preact/hooks'
import { invoke } from '@tauri-apps/api/core'
import { Checkbox } from '../components/Checkbox'

import './Main.css'

export function Main() {
  const [selected, setSelected] = useState('online')
  const [alwaysUse, setAlwaysUse] = useState(false)
  const [supportsOffline, setSupportsOffline] = useState(false)

  // Load the config
  useEffect(() => {
    (async () => {
      const config: Config = await invoke('get_config')
      const supportsOffline: boolean = await invoke('supports_offline')
      setSupportsOffline(supportsOffline)
      setAlwaysUse(config.skip_splash)
      setSelected(config.offline ? 'offline' : 'online')
    })()
  }, [])

  const setConfig = async (
    option: keyof Config,
    value: Config[keyof Config]
  ) => {
    const config = (await invoke('get_config')) as Config
    config[option] = value
    await invoke('write_config_file', {
      contents: JSON.stringify(config),
    })
  }

  return (
    <div class="card">
      <div class="mode-select">
        <div
          class={'mode ' + (selected === 'online' ? 'selected' : '')}
          id="online"
          onClick={() => {
            setSelected('online')
            setConfig('offline', false)
          }}
        >
          <span class="mode-title">Online (RECOMMENDED)</span>

          <div class="mode-img">
            <img src="arrow.svg" alt="Offline" />
          </div>
        </div>

        {supportsOffline && (
          <div
            class={'mode ' + (selected === 'offline' ? 'selected' : '')}
            id="offline"
            onClick={() => {
              setSelected('offline')
              setConfig('offline', true)
            }}
          >
            <span class="mode-title">Offline (LOCAL)</span>

            <div class="mode-img">
              <img src="arrow.svg" alt="Offline" />
            </div>
          </div>
        )}

        <div class="mode-lock">
          <Checkbox
            initialChecked={alwaysUse}
            id="always-use"
            label="Always use this mode"
            onChange={() => {
              setAlwaysUse(!alwaysUse)
              setConfig('skip_splash', !alwaysUse)
            }}
          />
        </div>
      </div>
      <div
        id="play"
        class="button"
        onClick={() => {
          invoke('launch')
        }}
      >
        <button>Play</button>
      </div>
    </div>
  )
}
