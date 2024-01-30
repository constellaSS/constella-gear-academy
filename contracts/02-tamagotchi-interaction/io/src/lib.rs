#![no_std]

use gmeta::{In, InOut, Metadata, Out};
use gstd::{exec, prelude::*, ActorId};

const HUNGER_PER_BLOCK: u64 = 1;
const BOREDOM_PER_BLOCK: u64 = 2;
const ENERGY_PER_BLOCK: u64 = 2;

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
        self.slept = self
            .slept
            .saturating_sub((self.slept_block - exec::block_height() as u64) * ENERGY_PER_BLOCK);
    }
    pub fn update_entertained(&mut self) {
        self.entertained = self.entertained.saturating_sub(
            (self.entertained_block - exec::block_height() as u64) * BOREDOM_PER_BLOCK,
        );
    }
    pub fn update_fed(&mut self) {
        self.fed = self
            .fed
            .saturating_sub((self.fed_block - exec::block_height() as u64) * HUNGER_PER_BLOCK);
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
