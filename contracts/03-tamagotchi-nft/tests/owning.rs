use gtest::{Log, Program, System};
use tamagotchi_nft_io::{TmgAction, TmgEvent};

// TODO: 0️⃣ Copy tests from the previous lesson and push changes to the master branch
const TEST_AGE: u64 = 30;
const OWNER_ACCOUNT: u64 = 2;
const APPROVED_ACCOUNT: u64 = 3;
const NEW_OWNER_ACCOUNT: u64 = 4;

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

    let res = _program.send(2, String::from("Tamagotchi Name"));
    assert!(!res.main_failed());

    let res = _program.send(2, TmgAction::Feed);
    let expected_log = Log::builder().dest(2).payload(TmgEvent::Fed);
    assert!(res.contains(&expected_log));

    let res = _program.send(2, TmgAction::Entertain);
    let expected_log = Log::builder().dest(2).payload(TmgEvent::Entertained);
    assert!(res.contains(&expected_log));

    let res = _program.send(2, TmgAction::Sleep);
    let expected_log = Log::builder().dest(2).payload(TmgEvent::Slept);
    assert!(res.contains(&expected_log));
}

#[test]
fn owning_test() {
    let sys = System::new();
    sys.init_logger();
    let _program = Program::current(&sys);

    // TODO: 6️⃣ Test new functionality

    // Test init the program
    let res = _program.send(OWNER_ACCOUNT, String::from("Tamagotchi Name"));
    assert!(!res.main_failed());

    // Test approve an account
    let res = _program.send(OWNER_ACCOUNT, TmgAction::Approve(APPROVED_ACCOUNT.into()));
    let expected_log = Log::builder()
        .dest(OWNER_ACCOUNT)
        .payload(TmgEvent::Approved(3.into()));
    assert!(res.contains(&expected_log));

    // Test transfer from owner
    let res = _program.send(OWNER_ACCOUNT, TmgAction::Transfer(NEW_OWNER_ACCOUNT.into()));
    let expected_log = Log::builder()
        .dest(OWNER_ACCOUNT)
        .payload(TmgEvent::Transferred(NEW_OWNER_ACCOUNT.into()));
    assert!(res.contains(&expected_log));

    // Test transfer from approved account
    let res = _program.send(APPROVED_ACCOUNT, TmgAction::Transfer(OWNER_ACCOUNT.into()));
    let expected_log = Log::builder()
        .dest(APPROVED_ACCOUNT)
        .payload(TmgEvent::Transferred(OWNER_ACCOUNT.into()));
    assert!(res.contains(&expected_log));

    // Test revoke approval
    let res = _program.send(OWNER_ACCOUNT, TmgAction::RevokeApproval);
    let expected_log = Log::builder()
        .dest(OWNER_ACCOUNT)
        .payload(TmgEvent::ApprovalRevoked);
    assert!(res.contains(&expected_log));
}
