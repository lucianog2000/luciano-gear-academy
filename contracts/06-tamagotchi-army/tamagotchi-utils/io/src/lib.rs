#![no_std]
use gmeta::{In, InOut, Metadata, Out};
use gstd::{prelude::*, ActorId, ReservationId};
use store_io::{AttributeId, TransactionId};

#[derive(Default, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct Tamagotchi {
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
    pub reservations: Vec<ReservationId>,
}

#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum TmgAction {
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
    CheckState,
    ReserveGas {
        reservation_amount: u64,
        duration: u32,
    },
}

#[derive(Encode, Decode, TypeInfo, Eq, PartialEq)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum TmgEvent {
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
    FeedMe,
    PlayWithMe,
    WantToSleep,
    NothingToDo,
    MakeReservation,
    GasReserved,
}

pub struct GasReservationHandler {
    pub contract_send_a_delayed_message: bool,
    pub can_send_delayed_message: bool,
}

#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct TmgInit {
    pub owner: ActorId,
    pub name: String,
}

pub struct ProgramMetadata;

impl Metadata for ProgramMetadata {
    type Init = In<String>;
    type Reply = ();
    type Others = InOut<TmgAction, TmgEvent>;
    type Signal = ();
    type Handle = InOut<TmgAction, TmgEvent>;
    type State = Out<Tamagotchi>;
}
