use gtest::{Program, System};

// TODO: 0️⃣ Copy tests from the previous lesson and push changes to the master branch

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
    let expected_log = Log::builder().dest(2).payload(String::from("Luchex"));
    assert!(res.contains(&expected_log));
}

#[test]
fn interaction_test() {
    let sys = System::new();
    sys.init_logger();
    let _program = Program::current(&sys);

    // TODO: 6️⃣ Test new functionality
}
