use amiquip::{Connection, Publish, Result};
use std::error::Error;
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn Error>> {
    let mut connection = Connection::insecure_open("amqp://centralReader:12345@localhost:5672")?;
    
    let channel = connection.open_channel(None)?;

    let mut ic = 1;
    let mut rt_key = "";

    loop {
        let payload = format!("Mensagem que vai para ... {}", ic);

        if ic % 2 == 0 {
            rt_key = "parkey";
        } else {
            rt_key = "imparkey";
        }

        let message = Publish::new(payload.as_bytes(), rt_key);

        let _  = channel.basic_publish("centralEx", message);
        
        //println!("Vou dormir...\n");
        thread::sleep(Duration::from_secs(1));
        //println!("Acordei !\n");
        ic = ic+1;
    }

}
