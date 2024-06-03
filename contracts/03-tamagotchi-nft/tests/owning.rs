use gstd::ActorId;
use gtest::{Log, Program, System};
use tamagotchi_nft_io::{TmgAction, TmgEvent};
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
    let program = Program::current(&sys);

    let init_response = program.send(2, String::from("Luchex"));
    assert!(!init_response.main_failed());

    let feed_response = program.send(2, TmgAction::Feed);
    assert!(!feed_response.log().is_empty());

    let entertain_response = program.send(2, TmgAction::Entertain);
    assert!(!entertain_response.log().is_empty());

    let sleep_response = program.send(2, TmgAction::Sleep);
    assert!(!sleep_response.log().is_empty());
}

#[test]
fn owning_test() {
    let sys = System::new();
    sys.init_logger();
    let program = Program::current(&sys);

    let init_response = program.send(2, String::from("Luchex"));
    assert!(!init_response.main_failed());

    let transfer_response = program.send(2, TmgAction::Transfer(ActorId::from(1234)));
    let expected_log = Log::builder()
        .dest(2)
        .payload(TmgEvent::Transferred(ActorId::from(1234)));
    assert!(transfer_response.contains(&expected_log));

    let approve_response = program.send(1234, TmgAction::Approve(ActorId::from(5678)));
    let expected_log = Log::builder()
        .dest(1234)
        .payload(TmgEvent::Approved(ActorId::from(5678)));
    assert!(approve_response.contains(&expected_log));

    let revoke_approval_response = program.send(1234, TmgAction::RevokeApproval);
    let expected_log = Log::builder().dest(1234).payload(TmgEvent::ApprovalRevoked);
    assert!(revoke_approval_response.contains(&expected_log));
}
