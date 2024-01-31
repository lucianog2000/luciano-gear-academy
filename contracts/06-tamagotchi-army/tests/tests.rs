#[allow(unused_imports)]
use gstd::ActorId;
#[allow(unused_imports)]
use gtest::{Log, Program, System, TestError};
#[allow(unused_imports)]
use tamagotchi_army_io::*;

#[test]
fn init_escrow_factory() {
    let system = System::new();
    let tamagotchi_code_id =
        system.submit_code("../target/wasm32-unknown-unknown/release/tamagotchi_auto.opt.wasm");
    let tamagotchi_factory = Program::current(&system);
    let res = tamagotchi_factory.send(100, tamagotchi_code_id);
    assert!(!res.main_failed());
}
