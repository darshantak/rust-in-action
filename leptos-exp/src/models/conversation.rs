use leptos::ev::message;
use serde::{Deserialize,Serialize};

#[derive(Deserialize,Serialize,Clone,Debug)]
pub struct Conversation{
    pub message: Vec<Message>
}

impl Conversation {
    pub fn new() -> Conversation{
        Conversation{
            message:Vec::new()
        }
    }
}

//user field will be true if the message is from human and false if the message is from the server
#[derive(Deserialize,Serialize,Clone,Debug)]
pub struct Message{
    pub user: bool,
    pub text: String
}

