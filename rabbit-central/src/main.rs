use amiquip::{Connection, Publish, Result};
use std::error::Error;
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn Error>> {
    let mut connection = Connection::insecure_open("amqp://centralReader:12345@localhost:5672")?;
    
    let channel = connection.open_channel(None)?;

    //for ic in 1..=10{
        let mut ic = 1;
    loop {
        let payload = format!("Hello, i love you! want tell your name ? {}", ic);
        let message = Publish::new(payload.as_bytes(), "");

        let _  = channel.basic_publish("centralEx", message);
        
        println!("Vou dormir...\n");
        thread::sleep(Duration::from_secs(10));
        println!("Acordei !\n");
        ic = ic+1;
    }

    //Ok(())
}
