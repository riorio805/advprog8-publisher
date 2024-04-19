use borsh::{BorshDeserialize, BorshSerialize};
use crosstown_bus::{CrosstownBus, MessageHandler, HandleError};

#[derive(Debug, Clone, BorshDeserialize, BorshSerialize)]
pub struct UserCreatedEventMessage {
    pub user_id: String,
    pub user_name: String
}

pub struct UserCreatedHandler;
impl MessageHandler<UserCreatedEventMessage> for UserCreatedHandler {
    fn handle(&self, message: Box<UserCreatedEventMessage>) ->
    Result<(), HandleError> {
        println!("Message received on handler 1: {:?}", message);
        Ok(())
    }
}

fn main() {
    let mut p =
        CrosstownBus::new_publisher("amqp://guest:guest@localhost:5672".to_owned()).unwrap();
    _ = p.send("user_created".to_owned(),
            UserCreatedEventMessage { user_id: "1".to_owned(), user_name:
            "2206818966-Amir".to_owned() });
    _ = p.send("user_created".to_owned(),
            UserCreatedEventMessage { user_id: "2".to_owned(), user_name:
            "2206818966-Budi".to_owned() });
    _ = p.send("user_created".to_owned(),
            UserCreatedEventMessage { user_id: "3".to_owned(), user_name:
            "2206818966-Cica".to_owned() });
    _ = p.send("user_created".to_owned(),
            UserCreatedEventMessage { user_id: "4".to_owned(), user_name:
            "2206818966-Dira".to_owned() });
    _ = p.send("user_created".to_owned(),
            UserCreatedEventMessage { user_id: "5".to_owned(), user_name:
            "2206818966-Emir".to_owned() });
}