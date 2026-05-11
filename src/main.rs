use borsh::{BorshDeserialize, BorshSerialize};
use crosstown_bus::{CrosstownBus, MessageHandler, HandleError};
use std::{thread, time};

#[derive(Debug, Clone, BorshDeserialize, BorshSerialize)]
pub struct UserCreatedEventMessage {
    pub user_id: String,
    pub user_name: String
}

pub struct UserCreatedHandler;

impl MessageHandler<UserCreatedEventMessage> for UserCreatedHandler {
    fn handle(&self, message: Box<UserCreatedEventMessage>) -> Result<(), HandleError> {
        let ten_millis = time::Duration::from_millis(1000);
        let _now = time::Instant::now();
        thread::sleep(ten_millis); 
        println!("In Rasyid's Computer [2406495786]. Message received: {:?}", message);
        Ok(())
    }
    
    fn get_handler_action(&self) -> String {
        String::from("user_created")
    }
}

fn main() {
    let url = "amqps://jilkwcrc:3B3qb2ONEYepKlWPrYqLcOVHrCXrz9S6@armadillo.rmq.cloudamqp.com/jilkwcrc".to_owned();
    
    let listener = CrosstownBus::new_queue_listener(url).unwrap();
    
    let _ = listener.listen("user_created".to_owned(), UserCreatedHandler{}, crosstown_bus::QueueProperties {
        auto_delete: false, durable: false, use_dead_letter: true
    });

    loop {}
}