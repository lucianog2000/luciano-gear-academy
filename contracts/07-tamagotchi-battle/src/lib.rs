#![no_std]
use gstd::{msg, prelude::*, ActorId};
use tamagotchi_battle_io::{Battle, BattleAction, BattleInit, BattleState};

static mut BATTLE: Option<Battle> = None;

#[no_mangle]
extern fn init() {
    let BattleInit { tmg_store_id } =
        msg::load().expect("Unable to decode CodeId of the Escrow program");
    let battle = Battle {
        players: Vec::new(),
        state: BattleState::Registration,
        current_turn: 0,
        tmg_store_id,
        winner: ActorId::default(),
        steps: 0,
    };

    unsafe {
        BATTLE = Some(battle);
    }
}

#[gstd::async_main]
async fn main() {
    let action: BattleAction = msg::load().expect("Unable to decode `BattleAction`");
    let battle: &mut Battle = unsafe { BATTLE.get_or_insert(Default::default()) };
    match action {
        BattleAction::Register(tamagotchi_id) => battle.register(&tamagotchi_id).await,
        BattleAction::Move(direction, combat_action) => battle.make_move(direction, combat_action),
        BattleAction::UpdateInfo => battle.update_info().await,
        BattleAction::ResetContract => battle.reset_contract(),
    }
}

#[no_mangle]
extern fn state() {
    let battle: Battle = unsafe { BATTLE.take().expect("Unexpected error in taking state") };
    msg::reply(battle, 0).expect("Failed to share state");
}
