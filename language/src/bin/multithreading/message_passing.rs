/* Message passing in Rust
what happenes in Arc is that we are sharing memory between multiple threads, so what does message passing do is that instead of sharing memory we share data/message between threads.
like there is queue and mulltiple threads can send message to the queue and one thread can recive the messages from the queue.
so it uses mpsc channel that is multiple producer single consumer channel, so multiple threads can send messages to the channel but only one thread can receive the messages from the channel.

we have to use sync::mpsc package for it.
for creating a channel : let (transmitter,receiver) = mpsc::channel();
for sending message: let status = transmitter.send(message);
for checking if message was sent successfully: if status.is_ok() {..impl..}

for reciving message: let recive_status = receiver.recv_timeout(Duration::from_millis(500));
for checking if message was recived successfully: if recive_status.is_ok() {..impl..}


*/

use std::{ sync::mpsc, time::Duration};
use std::thread;



fn main(){
    println!("Message Passing in Rust");

    let (transmitter,receiver) = mpsc::channel::<u8>();

    // let send_status = transmitter.send(10);
    // transmitter.send(20);
    // let recive_status = receiver.recv_timeout(Duration::from_millis(500));
    // println!("Recived message: {}",recive_status.unwrap());

    // let recive_status = receiver.recv_timeout(Duration::from_millis(500));
    // println!("Recived message: {}",recive_status.unwrap());

    // if send_status.is_ok(){
    //     println!("Message sent successfully");
    // }else{
    //     println!("Failed to send message");
    // }

    let recive_task = move || {

        println!("Starting the child thread");
        loop{
            println!("Waiting for message...");
        let recive_status= receiver.recv_timeout(Duration::from_millis(800));
        if recive_status.is_ok(){
            println!("Data recived:{}",recive_status.unwrap());
            }else{
                break;
            }
        }

    };

    for i in 0..10{
        let send_status = transmitter.send(i);
        println!("Sennd Status:{}",send_status.is_ok());
        thread::sleep(Duration::from_secs(1));
    }
    thread::spawn(recive_task).join().unwrap();



}