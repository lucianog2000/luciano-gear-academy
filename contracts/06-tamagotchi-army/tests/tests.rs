use gstd::ActorId;
use gtest::{Log, Program, System, TestError};
use tamagotchi_army_io::*;
use tamagotchi_utils_io::TmgEvent;

#[test]
fn tamagotchi_factory() {
    let system = System::new();
    let tamagotchi_code_id =
        system.submit_code("../target/wasm32-unknown-unknown/release/tamagotchi_auto.opt.wasm");
    let tamagotchi_factory = Program::current(&system);
    let init_response = tamagotchi_factory.send(100, tamagotchi_code_id);
    assert!(!init_response.main_failed());

    let create_response = tamagotchi_factory.send(
        2,
        TamagotchiFactoryAction::CreateTamagotchi {
            name: String::from("Luchex"),
        },
    );

    // let expected_log = Log::builder()
    //     .dest(2)
    //     .payload(TamagotchiFactoryEvent::TamagotchiCreated {
    //         tamagotchi_id: Default::default(),
    //         tamagotchi_address: Default::default(),
    //     });

    assert!(!create_response.main_failed());
    // assert!(create_response.contains(&expected_log));
}
