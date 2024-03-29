#![no_std]
use gstd::{msg, ActorId, prelude::*,Default};

#[derive(Debug, PartialEq, Eq)]
pub enum EscrowState {
    AwaitingPayment,
    AwaitingDelivery,
    Closed,
}

impl Default for EscrowState {
    fn default() -> Self {
        Self::AwaitingPayment
    }
}

pub struct Escrow {
    pub seller: ActorId,
    pub buyer: ActorId,
    pub price: u128,
    pub state: EscrowState,
}

#[derive(Encode, Decode, TypeInfo)]
pub struct InitEscrow {
    pub seller: ActorId,
    pub buyer: ActorId,
    pub price: u128,
}

static mut ESCROW: Option<Escrow> = None;

//enums from incoming and outgoing msgs
#[derive(Encode, Decode, TypeInfo)]
pub enum EscrowEvent {
    FundsDeposited,
    DeliveryConfirmed,
}


//extra added
#[derive(Encode, Decode, TypeInfo)]
pub enum EscrowAction {
    Deposit,
    ConfirmDelivery,
}

impl Escrow {
    fn deposit(&mut self) {}
    fn confirm_delivery(&mut self) {}
}

#[no_mangle]
extern "C" fn handle() {
    let action: EscrowAction = msg::load()
        .expect("Unable to decode `EscrowAction`");
    let escrow: &mut Escrow = unsafe {
        ESCROW
            .as_mut()
            .expect("The contract is not initialized")
    };
    match action {
        EscrowAction::Deposit => escrow.deposit(),
        EscrowAction::ConfirmDelivery => escrow.confirm_delivery(),
    }}

#[no_mangle]
extern "C" fn init() {
    let init_config: InitEscrow = msg::load()
        .expect("Error in decoding `InitEscrow`");
    let escrow = Escrow {
        seller: init_config.seller,
        buyer: init_config.buyer,
        price: init_config.price,
        state: EscrowState::AwaitingPayment,
    };
    unsafe { ESCROW = Some(escrow) };
}


#[no_mangle]
extern "C" fn state() {
    let escrow = unsafe {
        ESCROW.get_or_insert(Default::default())
    };
    msg::reply(escrow, 0).expect("Failed to share state");
}

#[no_mangle]
extern "C" fn metahash() {
    let metahash: [u8; 32] = include!("../.metahash");
    msg::reply(metahash, 0).expect("Failed to share metahash");
}