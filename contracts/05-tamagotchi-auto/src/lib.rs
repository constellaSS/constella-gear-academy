#![no_std]

use gstd::{async_main, msg};
#[allow(unused_imports)]
use gstd::{exec, prelude::*};
use tamagotchi_auto_io::{Tamagotchi, TmgAction, TmgEvent, MAX_STATUS_TMG_VALUE};

static mut TAMAGOTCHI: Option<Tamagotchi> = None;

#[no_mangle]
extern fn init() {
    // TODO: 0️⃣ Copy the `init` function from the previous lesson and push changes to the master branch
    let name: String = msg::load().expect("Can't decode the init message");

    let tamagotchi = Tamagotchi {
        name,
        date_of_birth: exec::block_timestamp(),
        owner: msg::source(),
        fed: MAX_STATUS_TMG_VALUE,
        fed_block: exec::block_height() as u64,
        entertained: MAX_STATUS_TMG_VALUE,
        entertained_block: exec::block_height() as u64,
        slept: MAX_STATUS_TMG_VALUE,
        slept_block: exec::block_height() as u64,
        approved_account: None,
        ft_contract_id: Default::default(),
        transaction_id: 0,
        approve_transaction: None,
        reservations: Default::default(),
    };

    unsafe { TAMAGOTCHI = Some(tamagotchi) }
}

#[async_main]
async fn main() {
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
        TmgAction::Sleep => {
            tmg.update_slept();
            tmg.sleep();
            msg::reply(TmgEvent::Slept, 0).expect("Error replying to the Sleep action");
        }
        TmgAction::Feed => {
            tmg.update_fed();
            tmg.feed();
            msg::reply(TmgEvent::Fed, 0).expect("Error replying to the Feed action");
        }
        TmgAction::Entertain => {
            tmg.update_entertained();
            tmg.entertain();
            msg::reply(TmgEvent::Entertained, 0).expect("Error replying to the Entertain action");
        }
        TmgAction::Approve(approved_account) => {
            if msg::source() != tmg.owner {
                panic!("Approve function is only available to the current owner of the Tamagotchi");
            }
            tmg.approved_account = Some(approved_account);
            msg::reply(TmgEvent::Approved(tmg.approved_account.unwrap()), 0)
                .expect("Error in sending Approved Event message");
        }
        TmgAction::Transfer(new_owner) => {
            let source = msg::source();
            if source != tmg.owner && source != tmg.approved_account.unwrap() {
                panic!("Transfer function is only available to the owner of the Tamagotchi or to the approved account");
            }
            tmg.owner = new_owner;
            msg::reply(TmgEvent::Transferred(tmg.owner), 0)
                .expect("Error in sending Transferred Event message");
        }
        TmgAction::RevokeApproval => {
            if msg::source() != tmg.owner {
                panic!("Approve function is only available to the current owner of the Tamagotchi");
            }
            tmg.approved_account = None;
            msg::reply(TmgEvent::ApprovalRevoked, 0)
                .expect("Error in sending ApprovalRevoked Event message");
        }
        TmgAction::SetFTokenContract(ft_contract_id) => {
            tmg.ft_contract_id = ft_contract_id;
            msg::reply(TmgEvent::FTokenContractSet, 0)
                .expect("Error replying to the SetFTokenContract Action");
        }
        TmgAction::ApproveTokens { account, amount } => tmg.approve_tokens(&account, amount).await,
        TmgAction::BuyAttribute {
            store_id,
            attribute_id,
        } => Tamagotchi::buy_attribute(store_id, attribute_id).await,
        TmgAction::CheckState => {}
        TmgAction::ReserveGas {
            reservation_amount: _,
            duration: _,
        } => {}
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
