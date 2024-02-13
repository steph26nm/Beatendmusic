use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Message {
    sender_id: u64,
    receiver_id: u64,
    content: String,
}

impl Message {
    fn new(sender_id: u64, receiver_id: u64, content: String) -> Self {
        Message {
            sender_id,
            receiver_id,
            content,
        }
    }
}

#[derive(Debug)]
struct SessionBooking {
    producer_id: u64,
    artist_id: u64,
    coach_id: Option<u64>,
    studio_id: Option<u64>,
    purpose: String,
    date: String,
    time: String,
    duration_hours: u32,
}

impl SessionBooking {
    fn new(
        producer_id: u64,
        artist_id: u64,
        coach_id: Option<u64>,
        studio_id: Option<u64>,
        purpose: String,
        date: String,
        time: String,
        duration_hours: u32,
    ) -> Self {
        SessionBooking {
            producer_id,
            artist_id,
            coach_id,
            studio_id,
            purpose,
            date,
            time,
            duration_hours,
        }
    }
}

#[derive(Debug, Clone)]
struct Entity {
    id: u64,
    name: String,
    purpose: String,
    inbox: Vec<Message>,
}

impl Entity {
    fn new(id: u64, name: String, purpose: String) -> Self {
        Entity {
            id,
            name,
            purpose,
            inbox: Vec::new(),
        }
    }

    fn send_message(&mut self, receiver: &mut Entity, content: String) {
        let message = Message::new(self.id, receiver.id, content.clone());
        self.inbox.push(message.clone());
        receiver.inbox.push(message);
    }

    fn read_inbox(&self) -> &Vec<Message> {
        &self.inbox
    }

    fn book_session(
        &self,
        artist_id: u64,
        coach_id: Option<u64>,
        studio_id: Option<u64>,
        purpose: String,
        date: String,
        time: String,
        duration_hours: u32,
    ) -> SessionBooking {
        SessionBooking::new(
            self.id,
            artist_id,
            coach_id,
            studio_id,
            purpose,
            date,
            time,
            duration_hours,
        )
    }
}
fn main() {
    let mut entities: HashMap<u64, Entity> = HashMap::new();

    // Create entities
    entities.insert(1, Entity::new(1, "Music Producer".to_string(), "Production".to_string()));
    entities.insert(2, Entity::new(2, "Beat Producer".to_string(), "Beat Making".to_string()));
    entities.insert(3, Entity::new(3, "Artist".to_string(), "Singer".to_string()));
    entities.insert(4, Entity::new(4, "Music Coach".to_string(), "Music Training".to_string()));
    entities.insert(5, Entity::new(5, "Recording Studio".to_string(), "Recording".to_string()));

    // Get mutable borrow of music producer
    if let Some(music_producer_entity) = entities.get_mut(&1) {
        // Get mutable borrow of artist
        if let Some(artist_entity) = entities.get_mut(&3) {
            // Clone the music producer entity and send a message
            let cloned_music_producer_entity = music_producer_entity.clone();
            
            cloned_music_producer_entity.send_message(artist_entity, "Hey, let's collaborate!".to_string());

            // Read artist's inbox
            let artist_inbox = artist_entity.read_inbox();
            println!("Artist Inbox:");
            for message in artist_inbox {
                println!("{:?}", message);
            }

            // Book session
            let session = music_producer_entity.book_session(
                3,
                Some(4),
                Some(5),
                "Recording Session".to_string(),
                "2024-03-15".to_string(),
                "10:00".to_string(),
                2,
            );
            println!("Session booked: {:?}", session);
        }
    }
}
