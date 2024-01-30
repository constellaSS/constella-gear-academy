#![no_std]

#[allow(unused_imports)]
use gstd::{exec, msg, prelude::*};
use tamagotchi_io::{Tamagotchi, TmgAction, TmgEvent};

static mut TAMAGOTCHI: Option<Tamagotchi> = None;

#[no_mangle]
extern fn init() {
    // TODO: 5️⃣ Initialize the Tamagotchi program
    let name: String = msg::load().expect("Can't decode the init message");

    let tamagotchi = Tamagotchi {
        name,
        date_of_birth: exec::block_timestamp(),
    };

    unsafe { TAMAGOTCHI = Some(tamagotchi) }
}

#[no_mangle]
extern fn handle() {
    // TODO: 6️⃣ Add handling of `Name` and `Age` actions
    let input_msg = msg::load().expect("Error in loading Tmg Input Message");
    let tmg = unsafe {
        TAMAGOTCHI
            .as_mut()
            .expect("The contract is not initialized")
    };
    match input_msg {
        TmgAction::Name => {
            msg::reply(TmgEvent::Name(tmg.name.clone()), 0).expect("Name not loaded correctly");
        }
        TmgAction::Age => {
            msg::reply(
                TmgEvent::Age(exec::block_timestamp() - tmg.date_of_birth),
                0,
            )
            .expect("Age not loaded correctly");
        }
    }
}

#[no_mangle]
extern fn state() {
    // TODO: 7️⃣ Return the Tamagotchi state
}
