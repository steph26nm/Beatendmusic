use candid::{CandidType, Deserialize};

use ic_cdk::export::candid::CandidType;
use std::collections::HashMap;

#[derive(Clone,Deserialize, CandidType)]
struct User {
    name: String,
    email: String,
}

#[derive(Clone, Deserialize, CandidType)]
struct Message {
    sender_id: User,
    receiver_id: User,
    content: String,
}

// HashMap to store messages between users
static mut MESSAGE_BOX: Option<HashMap<User, Vec<Message>>> = None;

fn initialize_message_box() {
    unsafe {
        MESSAGE_BOX = Some(HashMap::new());
    }
}

fn get_message_box() -> &'static mut HashMap<User, Vec<Message>> {
    unsafe {
        if let Some(ref mut messages) = MESSAGE_BOX {
            messages
        } else {
            initialize_message_box();
            MESSAGE_BOX.as_mut().unwrap()
        }
    }
}

#[ic_cdk::update]
fn backbeat_register(name: String, email: String) -> User {
    User { name, email }
}

#[ic_cdk::update]
fn send_message(sender_id: User, receiver_id: User, content: String) -> String {
    let message = Message {
        sender_id: sender_id.clone(),
        receiver_id: receiver_id.clone(),
        content,
    };

    let message_box = get_message_box();
    message_box.entry(sender_id).or_insert_with(Vec::new).push(message.clone());
    message_box.entry(receiver_id).or_insert_with(Vec::new).push(message);

    format!("Message sent from {} to {}: {}", sender_id.name, receiver_id.name, content)
}

#[ic_cdk::query]
fn get_messages(user: User) -> Vec<Message> {
    let message_box = get_message_box();
    if let Some(messages) = message_box.get(&user) {
        messages.clone()
    } else {
        Vec::new()
    }
}




