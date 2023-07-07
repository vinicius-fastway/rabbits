use amiquip::{Connection, ConsumerMessage, ConsumerOptions, QueueDeclareOptions, Result};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut connection = Connection::insecure_open("amqp://parReader:12345@localhost:5672")?;
    
    let channel = connection.open_channel(None)?;

    //let central_queue = channel.queue_declare("centralEx", QueueDeclareOptions::default())?;

    // Declare the exchange
    //let exchange = Exchange::direct(&channel);

    let central_queue = channel.queue_declare(
        "par",
        QueueDeclareOptions {
            durable: true,
            ..QueueDeclareOptions::default()
        },
    )?;

    //let central_queue = channel.queue_declare("par", QueueDeclareOptions::default())?;


    let consumer = central_queue.consume(ConsumerOptions::default())?;
    println!("CONSUMINDO....\n");
for (_i, message) in consumer.receiver().iter().enumerate() {
    //println!("{:?}", message);
    match message {
        ConsumerMessage::Delivery(delivery) => {
            // Process the message
            let body = String::from_utf8_lossy(&delivery.body);
            //println!("({:>3}) Recebido [{}]", i, body);
            println!("Recebido [{}]", body);

            // Acknowledge the message
            consumer.ack(delivery)?;
        }
        other => {
            println!("Fim: {:?}", other);
        }
        /*ConsumerMessage::ClientCancelled => break,
        ConsumerMessage::ServerCancelled => break,
        ConsumerMessage::ServerClosedChannel(_) => break,
        ConsumerMessage::ClientClosedChannel => break,
        ConsumerMessage::ClientClosedConnection => break,
        ConsumerMessage::ServerClosedConnection(_) => break,*/
    }
}

    //connection.close();
    Ok(())
}
