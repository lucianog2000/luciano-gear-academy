#![no_std]
use gmeta::{In, InOut, Metadata, Out};
use gstd::{prelude::*, ActorId};
use store_io::{AttributeId, TransactionId};

#[derive(Default, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct Tamagotchi {
    // TODO: 0️⃣ Copy fields from previous lesson and push changes to the master branch
    pub name: String,
    pub date_of_birth: u64,
    pub owner: ActorId,
    pub fed: u32,
    pub fed_block: u32,
    pub entertained: u32,
    pub entertained_block: u32,
    pub slept: u32,
    pub slept_block: u32,
    pub approved_account: Option<ActorId>,
    pub ft_contract_id: Option<ActorId>,
    pub transaction_id: u64,
    pub approve_transaction: Option<(TransactionId, ActorId, u128)>,
    // TODO: 1️⃣ Add new fields
}

#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum TmgAction {
    // TODO: 0️⃣ Copy actions from previous lesson and push changes to the master branch
    Name,
    Age,
    Feed,
    Entertain,
    Sleep,
    Transfer(ActorId),
    Approve(ActorId),
    RevokeApproval,
    SetFTokenContract(ActorId),
    ApproveTokens {
        account: ActorId,
        amount: u128,
    },
    BuyAttribute {
        store_id: ActorId,
        attribute_id: AttributeId,
    },
    // TODO: 2️⃣ Add new actions
}

#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum TmgEvent {
    // TODO: 0️⃣ Copy events from previous lesson and push changes to the master branch
    Name(String),
    Age(u64),
    Fed,
    Entertained,
    Slept,
    Transferred(ActorId),
    Approved(ActorId),
    ApprovalRevoked,
    FTokenContractSet,
    TokensApproved { account: ActorId, amount: u128 },
    ApprovalError,
    AttributeBought(AttributeId),
    CompletePrevPurchase(AttributeId),
    ErrorDuringPurchase,
    // TODO: 3️⃣ Add new events
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
