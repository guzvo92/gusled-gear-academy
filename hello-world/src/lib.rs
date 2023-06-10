#![no_std]
use gstd::{prelude::*,msg,exec};

#[derive(Default, Encode, Decode, TypeInfo)]
pub struct Tamagotchi {
   pub name: String,
   pub date_of_birth: u64,
}

static mut TAMAGOTCHI_MUT: Option<Tamagotchi> = None;

//firma
#[no_mangle]
extern "C" fn handle(){
    //let payload_string: String = msg::load().expect("Unable to decode `String`");
    let tmg = unsafe { TAMAGOTCHI_MUT.get_or_insert(Default::default()) };
    //let _nametamax = unsafe { NAMETAMA.as_ref().expect("The contract is not initialized")};
    msg::reply(tmg, 0).expect("Can't send a `SendHelloReply` message");
}

//firma
#[no_mangle]
extern "C" fn init(){ 
    let init_namemsg: String = msg::load().expect("Can't decode `InputMessage`");
    let init_age = exec::block_timestamp();
    let tmg = Tamagotchi {
        name: init_namemsg,
        date_of_birth: init_age,
    };
    unsafe {TAMAGOTCHI_MUT = Some(tmg)};   
    //debug!("Name init is {:?}",init_namemsg);   
}


#[no_mangle]
extern "C" fn state() {
    //let state = unsafe {NAMETAMA.as_ref().expect("The contract is not initialized")};
    let state_tmg = unsafe { TAMAGOTCHI_MUT.get_or_insert(Default::default()) };
    msg::reply(state_tmg, 0).expect("Unable to share the state");
}

//verify metadata for a program
#[no_mangle]
extern "C" fn metahash() {
    let metahash: [u8; 32] = include!("../.metahash");
    msg::reply(metahash, 0).expect("Unable to share the metahash");
}






