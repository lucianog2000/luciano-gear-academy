#![no_std]

#[allow(unused_imports)]
use gstd::{debug, exec, fmt, msg, prelude::*};
use tamagotchi_interaction_io::{Tamagotchi, TmgAction, TmgEvent};

static mut TAMAGOTCHI: Option<Tamagotchi> = None;

const HUNGER_PER_BLOCK: u32 = 1;
const BOREDOM_PER_BLOCK: u32 = 2;
const ENERGY_PER_BLOCK: u32 = 2;
const FILL_PER_FEED: u32 = 1000;
const FILL_PER_ENTERTAINMENT: u32 = 1000;
const FILL_PER_SLEEP: u32 = 1000;

#[no_mangle]
extern fn init() {
    let tamagotchi = Tamagotchi {
        name: msg::load().expect("Can't decode tamagotchi's name"),
        date_of_birth: exec::block_timestamp(),
        owner: msg::source(),
        fed: 9999,
        fed_block: exec::block_height(),
        entertained: 9999,
        entertained_block: exec::block_height(),
        slept: 9999,
        slept_block: exec::block_height(),
    };
    debug!(
        "The Tamagotchi Program was initialized with name {:?} and birth date {:?}",
        tamagotchi.name, tamagotchi.date_of_birth,
    );
    unsafe { TAMAGOTCHI = Some(tamagotchi) };
}

#[no_mangle]
extern fn handle() {
    let tamagotchi = unsafe {
        TAMAGOTCHI
            .as_mut()
            .expect("The contract is not initialized")
    };

    let tmg_action: TmgAction = msg::load().expect("Error loading TmgAction");
    match tmg_action {
        TmgAction::Name => reply_with(&tamagotchi.name),
        TmgAction::Age => {
            reply_with((exec::block_timestamp() - tamagotchi.date_of_birth).to_string())
        }
        TmgAction::Feed => {
            fill_stat_and_reply(
                &mut tamagotchi.fed,
                &mut tamagotchi.fed_block,
                HUNGER_PER_BLOCK,
                FILL_PER_FEED,
            );
            debug!("Feed action {:?}", &tamagotchi.fed.to_string());
        }
        TmgAction::Entertain => {
            fill_stat_and_reply(
                &mut tamagotchi.entertained,
                &mut tamagotchi.entertained_block,
                BOREDOM_PER_BLOCK,
                FILL_PER_ENTERTAINMENT,
            );
            debug!("Entertain action {:?}", &tamagotchi.entertained.to_string());
        }
        TmgAction::Sleep => {
            fill_stat_and_reply(
                &mut tamagotchi.slept,
                &mut tamagotchi.slept_block,
                ENERGY_PER_BLOCK,
                FILL_PER_SLEEP,
            );
            debug!("Sleep action {:?}", &tamagotchi.slept.to_string());
        }
    }
}

#[no_mangle]
extern fn state() {
    let tamagotchi = unsafe {
        TAMAGOTCHI
            .as_mut()
            .expect("The contract is not initialized")
    };
    update_stats(
        tamagotchi,
        tamagotchi.fed_block,
        tamagotchi.entertained_block,
        tamagotchi.slept_block,
    );
    debug!("Sending state: {:?}", tamagotchi);
    msg::reply(tamagotchi, 0).expect("Failed to share state");
}

fn reply_with<T: fmt::Display>(value: T) {
    msg::reply(&value.to_string(), 0).expect("Error in sending reply");
}

fn fill_stat_and_reply(
    stat: &mut u32,
    stat_block: &mut u32,
    stat_wasted_per_block: u32,
    fill_per_action: u32,
) {
    let actual_value = calculate_current_stat_value(*stat_block, stat_wasted_per_block);

    *stat = fill_stat_value(actual_value, fill_per_action);
    *stat_block = exec::block_height();

    reply_with(*stat);
}

fn calculate_current_stat_value(stat_block: u32, stat_wasted_per_block: u32) -> u32 {
    let stat_lost = (exec::block_height() - stat_block) * stat_wasted_per_block;

    let stat = 10000u32.saturating_sub(stat_lost).max(1).min(10000);

    stat
}

fn fill_stat_value(stat: u32, fill_per_action: u32) -> u32 {
    let filled_stat = stat + fill_per_action;
    filled_stat.max(1).min(10000)
}

fn update_stats(
    tamagotchi: &mut Tamagotchi,
    fed_block: u32,
    entertained_block: u32,
    slept_block: u32,
) {
    tamagotchi.fed -= calculate_current_stat_value(fed_block, HUNGER_PER_BLOCK)
        .max(1)
        .min(10000);
    tamagotchi.entertained -= calculate_current_stat_value(entertained_block, BOREDOM_PER_BLOCK)
        .max(1)
        .min(10000);
    tamagotchi.slept -= calculate_current_stat_value(slept_block, ENERGY_PER_BLOCK)
        .max(1)
        .min(10000);
}
