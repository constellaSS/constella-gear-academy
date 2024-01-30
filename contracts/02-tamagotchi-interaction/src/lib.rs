#![no_std]

#[allow(unused_imports)]
use gstd::{exec, msg, prelude::*};
use tamagotchi_interaction_io::{Tamagotchi, TmgAction, TmgEvent};

static mut TAMAGOTCHI: Option<Tamagotchi> = None;

// TODO: 4️⃣ Define constants
const FILL_PER_FEED: u64 = 1000;
const FILL_PER_ENTERTAINMENT: u64 = 1000;
const FILL_PER_SLEEP: u64 = 1000;

#[no_mangle]
extern fn init() {
    // TODO: 0️⃣ Copy the `init` function from the previous lesson and push changes to the master branch
    let name: String = msg::load().expect("Can't decode the init message");

    let tamagotchi = Tamagotchi {
        name,
        date_of_birth: exec::block_timestamp(),
        owner: msg::source(),
        fed: FILL_PER_SLEEP,
        fed_block: exec::block_height() as u64,
        entertained: FILL_PER_ENTERTAINMENT,
        entertained_block: exec::block_height() as u64,
        slept: FILL_PER_SLEEP,
        slept_block: exec::block_height() as u64,
    };

    unsafe { TAMAGOTCHI = Some(tamagotchi) }
}

#[no_mangle]
extern fn handle() {
    // TODO: 0️⃣ Copy the `handle` function from the previous lesson and push changes to the master branch
    let input_msg = msg::load().expect("Error in loading Tmg Input Message");
    let tmg = unsafe {
        TAMAGOTCHI
            .as_mut()
            .expect("The contract is not initialized")
    };
    match input_msg {
        TmgAction::Name => {
            msg::reply(TmgEvent::Name(tmg.name.clone()), 0)
                .expect("Error replying to the Name action");
        }
        TmgAction::Age => {
            msg::reply(
                TmgEvent::Age(exec::block_timestamp().saturating_sub(tmg.date_of_birth)),
                0,
            )
            .expect("Error replying to the Age action");
        }
        // TODO: 5️⃣ Add new logic for calculating the `fed`, `entertained` and `slept` levels
        TmgAction::Sleep => {
            tmg.update_slept();
            tmg.slept = tmg.slept.saturating_add(FILL_PER_SLEEP);
            tmg.slept_block = exec::block_height() as u64;
            msg::reply(TmgEvent::Slept, 0).expect("Error replying to the Sleep action");
        }
        TmgAction::Feed => {
            tmg.update_fed();
            tmg.fed = tmg.fed.saturating_add(FILL_PER_FEED);
            tmg.fed_block = exec::block_height() as u64;
            msg::reply(TmgEvent::Fed, 0).expect("Error replying to the Feed action");
        }
        TmgAction::Entertain => {
            tmg.update_entertained();
            tmg.entertained = tmg.entertained.saturating_add(FILL_PER_ENTERTAINMENT);
            tmg.entertained_block = exec::block_height() as u64;
            msg::reply(TmgEvent::Entertained, 0).expect("Error replying to the Entertain action");
        }
    }
}

#[no_mangle]
extern fn state() {
    // TODO: 0️⃣ Copy the `handle` function from the previous lesson and push changes to the master branch
    let tmg = unsafe {
        TAMAGOTCHI
            .as_ref()
            .expect("The contract is not initialized")
    };
    msg::reply(tmg, 0).expect("Failed to share state");
}
