#![no_std]                  //por default autocarga gstd standar y hay que darle esto para que no lo haga

//metadata son las estructuras de datos en el init/handle/handlereply
use gstd::{prelude::*,ActorId,Encode,Decode,TypeInfo,Debug};
use gmeta::{InOut, Metadata};
pub struct HelloMetadata;

#[derive(Encode, Decode, TypeInfo, Debug)]
pub enum InputMessageX {
    SendHelloTo(ActorId), 
    SendHelloReply,
}


impl Metadata for HelloMetadata {
    type Init = InOut<String, ()>;
    type Handle = InOut<InputMessageX, String>;
    type State = String;
    type Reply = (); 
    type Signal = ();
    type Others = (); 
 }