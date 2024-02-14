use gtest::{Program, System};

#[test]
fn init_escrow_factory() {
    let system = System::new();
    system.init_logger();
    let tamagotchi_code_id =
        system.submit_code("../target/wasm32-unknown-unknown/release/tamagotchi_auto.opt.wasm");
    let escrow_factory = Program::current(&system);
    let res = escrow_factory.send(100, tamagotchi_code_id);

    assert!(!res.main_failed());
}
