/* MUTEX - Mutual exclusion is a technique used to prevent multiple threads from accessing the same resource at the same time. 
This is important because if multiple threads access the same resource at the same time.

for creating a mutex , Mutex::new(data)
for accessing the data from mutex and obtaining the lock on the data: *(mutex_data).lock().unwrap() , make the variable mutable so that we can edit it.

use Arc and Mutex together 

*/

use std::{sync::{Arc, Mutex}, thread};

fn main(){

    let counter = Arc::new(Mutex::new(0));
    let mut handlers = vec![];

    for _ in 0..=11{
        let counter = Arc::clone(&counter);

        let handler = thread::spawn(
            move ||{
                let mut locked_counter = counter.lock().unwrap();
                *locked_counter+=1;
            }
        );

        handlers.push(handler);

    }

    for handler in handlers{
        handler.join().unwrap();
    }

    println!("Final counter value: {}",*counter.lock().unwrap());

}