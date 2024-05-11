export async function setInitialRPC() {
  // @ts-expect-error cry
  __TAURI_INTERNALS__.invoke('set_activity', {
    state: 'In the main menu',
    details: 'Playing PokeRogue',
    largeImage: 'meadow',
    smallImage: 'pokeball'
  })
}