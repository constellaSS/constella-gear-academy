#![no_std]

use gmeta::{In, InOut, Metadata};
use gstd::prog::ProgramGenerator;
use gstd::{debug, msg, prelude::*, ActorId, CodeId};

const GAS_FOR_CREATION: u64 = 1_000_000_000;

pub type TmgId = u64;

#[derive(Default)]
pub struct TamagotchiFactory {
    pub tamagotchi_number: u64,
    pub id_to_address: collections::BTreeMap<TmgId, ActorId>,
    pub tamagotchi_code_id: CodeId,
}

impl TamagotchiFactory {
    pub async fn create_tamagotchi(&mut self, name: String) {
        let (address, _) = ProgramGenerator::create_program_with_gas_for_reply(
            self.tamagotchi_code_id,
            name.encode(),
            GAS_FOR_CREATION,
            0,
            0,
        )
        .expect("Error during Tamagotchi program initialization")
        .await
        .expect("Program was not initialized");

        debug!("{:?}", address);
        self.tamagotchi_number = self.tamagotchi_number.saturating_add(1);
        self.id_to_address.insert(self.tamagotchi_number, address);
        msg::reply(FactoryEvent::TamagotchiCreated(name), 0)
            .expect("Error during a reply `FactoryEvent::TamagotchiCreated`");
    }
}

#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum FactoryAction {
    CreateTamagotchi { name: String },
}

#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum FactoryEvent {
    TamagotchiCreated(String),
}

pub struct ProgramMetadata;

// TODO: 0️⃣ Copy `Metadata` from the first lesson and push changes to the master branch
impl Metadata for ProgramMetadata {
    type Init = In<CodeId>;
    type Handle = InOut<FactoryAction, FactoryEvent>;
    type Reply = ();
    type Others = ();
    type Signal = ();
    type State = ();
}
