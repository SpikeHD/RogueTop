use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const SESSION_SLOT_COUNT: usize = 5;

#[derive(Debug, Serialize, Deserialize)]
struct SystemSaveData {
    trainer_id: i32,
    secret_id: i32,
    gender: i32,
    dex_data: DexData,
    starter_data: StarterData,
    starter_move_data: StarterMoveData,
    starter_egg_move_data: StarterEggMoveData,
    game_stats: GameStats,
    unlocks: Unlocks,
    achv_unlocks: AchvUnlocks,
    voucher_unlocks: VoucherUnlocks,
    voucher_counts: VoucherCounts,
    eggs: Vec<EggData>,
    game_version: String,
    timestamp: i32,
}

type DexData = HashMap<i32, DexEntry>;

#[derive(Debug, Serialize, Deserialize)]
struct DexEntry {
    seen_attr: serde_json::Value, // Deserialized as integer or string
    caught_attr: serde_json::Value, // Deserialized as integer or string
    nature_attr: i32,
    seen_count: i32,
    caught_count: i32,
    hatched_count: i32,
    ivs: Vec<i32>,
}

type StarterData = HashMap<i32, StarterEntry>;

#[derive(Debug, Serialize, Deserialize)]
struct StarterEntry {
    moveset: serde_json::Value,
    egg_moves: i32,
    candy_count: i32,
    friendship: i32,
    ability_attr: i32,
    passive_attr: i32,
    value_reduction: i32,
    classic_win_count: i32,
}

type StarterMoveData = HashMap<i32, serde_json::Value>;

type StarterEggMoveData = HashMap<i32, i32>;

type GameStats = serde_json::Value;

type Unlocks = HashMap<i32, bool>;

type AchvUnlocks = HashMap<String, i32>;

type VoucherUnlocks = HashMap<String, i32>;

type VoucherCounts = HashMap<String, i32>;

#[derive(Debug, Serialize, Deserialize)]
struct EggData {
    id: i32,
    gacha_type: GachaType,
    hatch_waves: i32,
    timestamp: i32,
}

#[derive(Debug, Serialize, Deserialize)]
enum GachaType {
    // Define your enum variants here if needed
}

#[derive(Debug, Serialize, Deserialize)]
struct SessionSaveData {
    seed: String,
    play_time: i32,
    game_mode: GameMode,
    party: Vec<PokemonData>,
    enemy_party: Vec<PokemonData>,
    modifiers: Vec<PersistentModifierData>,
    enemy_modifiers: Vec<PersistentModifierData>,
    arena: ArenaData,
    pokeball_counts: PokeballCounts,
    money: i32,
    score: i32,
    victory_count: i32,
    faint_count: i32,
    revive_count: i32,
    wave_index: i32,
    battle_type: BattleType,
    trainer: TrainerData,
    game_version: String,
    timestamp: i32,
}

#[derive(Debug, Serialize, Deserialize)]
enum GameMode {
  // TODO
}

type PokemonData = serde_json::Value;

type PersistentModifierData = serde_json::Value;

type ArenaData = serde_json::Value;

type PokeballCounts = HashMap<String, i32>;

#[derive(Debug, Serialize, Deserialize)]
enum BattleType {
  TODO
}

type TrainerData = serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
struct SessionHistoryData {
    seed: String,
    play_time: i32,
    result: SessionHistoryResult,
    game_mode: GameMode,
    party: Vec<PokemonData>,
    modifiers: Vec<PersistentModifierData>,
    money: i32,
    score: i32,
    wave_index: i32,
    battle_type: BattleType,
    game_version: String,
    timestamp: i32,
}

#[derive(Debug, Serialize, Deserialize)]
enum SessionHistoryResult {
  // TODO
}