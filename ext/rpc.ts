interface SessionData {
  seed: string
  playTime: number
  money: number
  score: number
  waveIndex: number

  enemyParty: {
    id: string
    species: string
    level: number
    boss: boolean
  }[]
}

function getMainData() {
  const encodedData = localStorage.getItem('data')
  const data = encodedData ? JSON.parse(atob(encodedData)) : null

  return data
}

function getSessionData() {
  const encodedSessionData = localStorage.getItem('sessionData')
  const sessionData: SessionData | null = encodedSessionData ? JSON.parse(atob(encodedSessionData)) : null

  return sessionData
}

export async function startRPCTracker() {
  // @ts-expect-error cry
  const enabled = await __TAURI_INTERNALS__.invoke('rpc_enabled')

  if (!enabled) return

  const mainData = await getMainData()
  const gameVersion = mainData?.gameVersion

  // Every 5 seconds, read the session data and update the RPC accordingly
  setInterval(async () => {
    const sessionData = getSessionData()

    // @ts-expect-error cry
    __TAURI_INTERNALS__.invoke('set_activity', {
      state: `PokéRogue ${gameVersion}`,
      details: `Fighting ${sessionData?.enemyParty.length ?? 0} ${sessionData?.enemyParty.length ?? 0 > 0 ? sessionData?.enemyParty[0].boss ? 'boss Pokémon' : 'Pokémon' : 'Pokémon'} | Floor ${sessionData?.waveIndex ?? 0}`,
      largeImage: 'meadow',
      smallImage: 'pokeball'
    })
  }, 5000)
}