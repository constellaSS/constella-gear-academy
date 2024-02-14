#![no_std]
use gstd::{msg, prelude::*, CodeId};
use tamagotchi_army_io::{FactoryAction, TamagotchiFactory};

static mut ESCROW_FACTORY: Option<TamagotchiFactory> = None;

#[gstd::async_main]
async fn main() {
    let action: FactoryAction = msg::load().expect("Unable to decode `FactoryAction`");
    let factory = unsafe { ESCROW_FACTORY.get_or_insert(Default::default()) };
    match action {
        FactoryAction::CreateTamagotchi { name } => factory.create_tamagotchi(name).await,
    }
}

#[no_mangle]
extern fn init() {
    let escrow_code_id: CodeId =
        msg::load().expect("Unable to decode CodeId of the Escrow program");
    let escrow_factory = TamagotchiFactory {
        tamagotchi_code_id: escrow_code_id,
        ..Default::default()
    };
    unsafe { ESCROW_FACTORY = Some(escrow_factory) };
}
