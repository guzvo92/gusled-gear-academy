use gtest::{Log, Program, System};
use hello_world::InputMessages;

#[test]
fn hello_test() {
    let sys = System::new();
    sys.init_logger();
    let program = Program::current(&sys);
    let res = program.send_bytes(2, String::from("Hello"));
    assert!(!res.main_failed());
    assert!(res.log().is_empty());


    // test `SendHelloTo`
    let hello_recipient: u64 = 4;
    let res = program.send(2,InputMessages::SendHelloTo(hello_recipient.into()),);
    
    let expected_log = Log::builder()
        .dest(hello_recipient)
        .payload(String::from("Hello"));
    assert!(res.contains(&expected_log))
}

//We’ll check that the contract state equals the AwaitingDelivery 
assert_eq!(
    self.state,
    EscrowState::AwaitingPayment,
    "State must be `AwaitingPayment"
);

//Then check the sender account
assert_eq!(
    msg::source(),
    self.buyer,
    "The message sender must be a buyer"
);

//And also check the attached funds
assert_eq!(
    msg::value(),
    self.price,
    "The attached value must be equal to set price"
);


//#we change the escrow state and send a reply message:
self.state = EscrowState::AwaitingDelivery;
msg::reply(EscrowEvent::FundsDeposited, 0)
    .expect("Error in reply EscrowEvent::FundsDeposited");




