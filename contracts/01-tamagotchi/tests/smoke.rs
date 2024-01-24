use gtest::{Log, Program, System};
use tamagotchi_io::{TmgAction, TmgEvent};
#[test]
fn smoke_test() {
    // test contract initialization
    let sys = System::new();
    sys.init_logger();
    let program = Program::current(&sys);
    let res = program.send(2, String::from("Luchex"));
    assert!(!res.main_failed());

    // test handling name
    let res = program.send(2, TmgAction::Name);
    let expected_log = Log::builder()
        .dest(2)
        .payload(TmgEvent::Name("Luchex".to_string()));
    assert!(res.contains(&expected_log));

    // test handling age
    let res = program.send(2, TmgAction::Age);
    assert!(!res.main_failed());
}
