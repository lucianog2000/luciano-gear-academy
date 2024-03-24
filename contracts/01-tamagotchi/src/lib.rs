#![no_std]

#[allow(unused_imports)]
use gstd::{debug, exec, msg, prelude::*};
use tamagotchi_io::{Tamagotchi, TmgAction, TmgEvent};

static mut TAMAGOTCHI: Option<Tamagotchi> = None;

#[no_mangle]
extern fn init() {
    let tamagotchi = Tamagotchi {
        name: msg::load().expect("Can't decode tamagotchi's name"),
        date_of_birth: exec::block_timestamp(),
    };
    debug!(
        "The Tamagotchi Program was initialized with name {:?} and birth date {:?}",
        tamagotchi.name, tamagotchi.date_of_birth
    );
    unsafe { TAMAGOTCHI = Some(tamagotchi) };
}

#[no_mangle]
extern fn handle() {
    let tamagotchi = unsafe {
        TAMAGOTCHI
            .as_ref()
            .expect("The contract is not initialized")
    };

    let name = &tamagotchi.name;
    let current_time = exec::block_timestamp();
    let age = current_time - tamagotchi.date_of_birth;
    let tmg_action: TmgAction = msg::load().expect("Error loading TmgAction");
    match tmg_action {
        TmgAction::Name => {
            debug!("Message: Name");
            msg::reply(TmgEvent::Name(name.to_string()), 0)
                .expect("Error in sending tamagotchi's name");
        }
        TmgAction::Age => {
            debug!("Message: Age");
            msg::reply(TmgEvent::Age(age), 0).expect("Error in sending tamagotchi's age");
        }
    }
}

#[no_mangle]
extern fn state() {
    let tamagotchi = unsafe {
        TAMAGOTCHI
            .as_ref()
            .expect("The contract is not initialized")
    };
    msg::reply(tamagotchi, 0).expect("Failed to share state");
}
