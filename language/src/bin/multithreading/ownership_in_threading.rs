/*

Ownership in Threading.
ok so sometimes there is a peice of data of whom the main thread is the owner and if the thread wants to use that data , it needs to borrow it so there comes the borrow error.
so to solve this issue, we transfer the ownership of the data to the thread using the move keyword and then the main thread looses the ownership of the data.

syntax: 

let worker_task = move ||{
    // code that uses the data
};  

let handler = thread::spawn(worker_task);

handler.join().unwrap();

if there are multiple threads that need to use the same data than we can clone the data using clone method and give every thread seprate copy 


*/

use std::{thread, time::Duration, vec};

fn main(){
    let vector = vec![1,2,3];

    let vec1 = vector.clone();
    let vec2 = vector.clone();

    let worker_task1 = move ||{
        thread::sleep(Duration::from_secs(2));
        println!("Worker thread: {:?}", vec1);
    };

    let worker_task2 = move ||{
        thread::sleep(Duration::from_secs(2));
        println!("Worker thread: {:?}", vec2);
    };

    let handler1 = thread::spawn(worker_task1);
    let handler2 = thread::spawn(worker_task2); 

    handler1.join().unwrap();
    handler2.join().unwrap();

    println!("Main Thread:{:?}",vector);

}