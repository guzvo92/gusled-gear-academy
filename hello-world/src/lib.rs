#![no_std]                  //no quiero usar la libreria std porque voy a implementar unalibreria estandar en gear
use gstd::{prelude::*,msg,ActorId,debug};     // voy a querer todo (*) del elemento prelude de gstd

#[derive(Encode, Decode, TypeInfo, Debug)]
pub enum InputMessageX {
    SendHelloTo(ActorId),
    SendHelloReply,
}

static mut GREETING: Option<String> = None;


//firma
#[no_mangle]
extern "C" fn init(){
    let init_message: String = msg::load().expect("Can't load the incoming message");
    debug!("Program initialized {:?}", init_message); //la imprimo
    unsafe { GREETING = Some(init_message)}; //la inicializo a some 
}

//firma
#[no_mangle]
extern "C" fn handle(){

    let message: InputMessageX = msg::load().expect("Can't decode `InputMessage`");
    debug!("Incoming message {:?}", message); //la imprimo
    let greeting = unsafe { GREETING.as_ref().expect("The contract is not initialized")};

    //es como un switchcase
    match message {
        InputMessageX::SendHelloTo(account) => {
            msg::send(account, greeting, 0).expect("Can't send a `SendHelloTo` message");
        },
        InputMessageX::SendHelloReply => {
            msg::reply(greeting, 0).expect("Can't send a `SendHelloReply` message");
        }
    }

}

//verify metadata for a program
#[no_mangle]
extern "C" fn metahash() {
    let metahash: [u8; 32] = include!("../.metahash");
    msg::reply(metahash, 0).expect("Unable to share the metahash");
}

//consultar grreeeting lo asigna a state y lo contesta en reply
//cuando alguien consulkta el state lo regresa como respuesta
#[no_mangle]
extern "C" fn state() {
    let state = unsafe {GREETING.as_ref().expect("The contract is not initialized")};
    msg::reply(state, 0).expect("Unable to share the state");
}





