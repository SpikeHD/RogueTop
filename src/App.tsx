import { Main } from './chunks/Main'

import './App.css'
import { Mods } from './chunks/Mods'
import { useEffect, useState } from 'preact/hooks'
import { invoke } from '@tauri-apps/api/core'

function App() {
  const [mobilePlatforms, setMobilePlatforms] = useState({
    isMobile: false,
    isAndroid: false,
    isIOS: false,
  })

  useEffect(() => {
    (async () => {
      setMobilePlatforms({
        isMobile: await invoke('is_mobile'),
        isAndroid: await invoke('is_android'),
        isIOS: await invoke('is_ios'),
      })
    })()
  }, [])

  return (
    <div class="container">
      <Main />

      {
        // Might work on Android, definitely not on iOS
        !mobilePlatforms.isMobile && <Mods />
      }
    </div>
  )
}

export default App
