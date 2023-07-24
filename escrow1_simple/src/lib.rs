#![no_std]
use gstd::{msg, ActorId, prelude::*};

pub enum EscrowState {
    AwaitingPayment,
    AwaitingDelivery,
    Closed,
}

#[derive(Encode, Decode, TypeInfo)]
pub enum EscrowEvent {
    FundsDeposited,
    DeliveryConfirmed,
}

impl Default for EscrowState {
    fn default() -> Self {
        Self::AwaitingPayment
    }
}

//Next, let’s define the structure that will store all necessary states:
pub struct Escrow {
    seller: ActorId,
    buyer: ActorId,
    price: u128,
    state: EscrowState,
}

impl Escrow {
    fn deposit(&mut self) {}
    fn confirm_delivery(&mut self) {}
}

static mut ESCROW: Option<Escrow> = None;

#[derive(Encode, Decode, TypeInfo)]
pub struct InitEscrow {
    pub seller: ActorId,
    pub buyer: ActorId,
    pub price: u128,
}

#[no_mangle]
extern "C" fn init() {
    let init_config: InitEscrow = msg::load().expect("Error in decoding `InitEscrow`");
    let escrow = Escrow {
        seller: init_config.seller,
        buyer: init_config.buyer,
        price: init_config.price,
        state: EscrowState::AwaitingPayment,
    };
    unsafe { ESCROW = Some(escrow) };
}

#[no_mangle]
extern "C" fn handle() {
    let action: EscrowEvent = msg::load().expect("Unable to decode `EscrowAction`");
    let escrow: &mut Escrow = unsafe {
        ESCROW.as_mut().expect("The contract is not initialized")
    };
    match action {
        EscrowEvent::FundsDeposited => escrow.deposit(),
        EscrowEvent::DeliveryConfirmed => escrow.confirm_delivery(),
    }
}
