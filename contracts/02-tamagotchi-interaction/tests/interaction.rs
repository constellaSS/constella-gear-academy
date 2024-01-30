use gtest::{Log, Program, System};
use tamagotchi_interaction_io::{TmgAction, TmgEvent};

// TODO: 0️⃣ Copy tests from the previous lesson and push changes to the master branch
const TEST_AGE: u64 = 30;

#[test]
fn smoke_test() {
    let sys = System::new();
    sys.init_logger();
    let _program = Program::current(&sys);

    let res = _program.send(2, String::from("Tamagotchi Name"));
    assert!(!res.main_failed());

    let res = _program.send(2, TmgAction::Name);
    let expected_log = Log::builder()
        .dest(2)
        .payload(TmgEvent::Name(String::from("Tamagotchi Name")));
    assert!(res.contains(&expected_log));

    sys.spend_blocks(TEST_AGE as u32);
    let res = _program.send(2, TmgAction::Age);
    let expected_log = Log::builder()
        .dest(2)
        .payload(TmgEvent::Age(TEST_AGE * 1000));
    assert!(res.contains(&expected_log));
}

#[test]
fn interaction_test() {
    let sys = System::new();
    sys.init_logger();
    let _program = Program::current(&sys);

    // TODO: 6️⃣ Test new functionality
}
