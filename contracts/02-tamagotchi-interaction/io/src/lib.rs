#![no_std]

use gmeta::{In, InOut, Metadata, Out};
use gstd::{exec, prelude::*, ActorId};

// TODO: 4️⃣ Define constants
pub const MAX_STATUS_TMG_VALUE: u64 = 10_000;
const HUNGER_PER_BLOCK: u64 = 1;
const BOREDOM_PER_BLOCK: u64 = 2;
const ENERGY_PER_BLOCK: u64 = 2;
const FILL_PER_FEED: u64 = 1000;
const FILL_PER_ENTERTAINMENT: u64 = 1000;
const FILL_PER_SLEEP: u64 = 1000;

#[derive(Default, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct Tamagotchi {
    // TODO: 0️⃣ Copy fields from previous lesson and push changes to the master branch
    pub name: String,
    pub date_of_birth: u64,
    // TODO: 1️⃣ Add new fields
    pub owner: ActorId,
    pub fed: u64,
    pub fed_block: u64,
    pub entertained: u64,
    pub entertained_block: u64,
    pub slept: u64,
    pub slept_block: u64,
}

impl Tamagotchi {
    pub fn update_slept(&mut self) {
        let new_slept = self
            .slept
            .saturating_sub((exec::block_height() as u64 - self.slept_block) * ENERGY_PER_BLOCK);
        if new_slept == 0 {
            self.slept = 1;
        } else {
            self.slept = new_slept;
        }
    }
    pub fn sleep(&mut self) {
        let new_slept = self.slept.saturating_add(FILL_PER_SLEEP);
        if new_slept > MAX_STATUS_TMG_VALUE {
            self.slept = MAX_STATUS_TMG_VALUE;
        } else {
            self.slept = new_slept;
        }
        self.slept_block = exec::block_height() as u64;
    }
    pub fn update_entertained(&mut self) {
        let new_entertained = self.entertained.saturating_sub(
            (exec::block_height() as u64 - self.entertained_block) * BOREDOM_PER_BLOCK,
        );
        if new_entertained == 0 {
            self.entertained = 1;
        } else {
            self.entertained = new_entertained;
        }
    }
    pub fn entertain(&mut self) {
        let new_entertained = self.entertained.saturating_add(FILL_PER_ENTERTAINMENT);
        if new_entertained > MAX_STATUS_TMG_VALUE {
            self.entertained = MAX_STATUS_TMG_VALUE;
        } else {
            self.entertained = new_entertained;
        }
        self.entertained_block = exec::block_height() as u64;
    }
    pub fn update_fed(&mut self) {
        let new_fed = self
            .fed
            .saturating_sub((exec::block_height() as u64 - self.fed_block) * HUNGER_PER_BLOCK);
        if new_fed == 0 {
            self.fed = 1;
        } else {
            self.fed = new_fed;
        }
    }
    pub fn feed(&mut self) {
        let new_fed = self.fed.saturating_add(FILL_PER_FEED);
        if new_fed > MAX_STATUS_TMG_VALUE {
            self.fed = MAX_STATUS_TMG_VALUE;
        } else {
            self.fed = new_fed;
        }
        self.fed_block = exec::block_height() as u64;
    }
}

#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum TmgAction {
    // TODO: 0️⃣ Copy actions from previous lesson and push changes to the master branch
    Name,
    Age,
    // TODO: 2️⃣ Add new actions
    Feed,
    Entertain,
    Sleep,
}

#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum TmgEvent {
    // TODO: 0️⃣ Copy events from previous lesson and push changes to the master branch
    Name(String),
    Age(u64),
    // TODO: 3️⃣ Add new events
    Fed,
    Entertained,
    Slept,
}

pub struct ProgramMetadata;

// TODO: 0️⃣ Copy `Metadata` from the first lesson and push changes to the master branch
impl Metadata for ProgramMetadata {
    type Init = In<String>;
    type Handle = InOut<TmgAction, TmgEvent>;
    type State = Out<Tamagotchi>;
    type Reply = ();
    type Others = ();
    type Signal = ();
}
