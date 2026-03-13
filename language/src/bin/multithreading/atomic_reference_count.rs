/* ARC- Atomic Reference Count

if there is a data of 10 mb than it will be very memory consuming if we create multiple copies of it , for every new thread or server.
so instead of this we use Arc which creates a global copy of the data which will be used by everyone , and gives a new reference address whenever someone asks the data.
Arc is thread safe and it uses atomic reference counting to keep track of the number of references to the data. When the reference count drops to zero, the data is deallocated.


1) Arc::new(&data)  to create a global copy of the data to which all can refer.
2) Arc::clone(&data) to create a clone of the data which can be used by multiple threads without creating multiple copies of the data.
3) Arc::strong_count(&data) to get the number of references to the data currently. 

*/

use std::{sync::Arc, thread};

fn main(){
println!("Atomic Reference Count");

// Creating a big data of 1 mb and creating a global copy of it using Arc::new(data)
let vector = vec![1,1000000];
let data = Arc::new(vector);
println!("Size of vector is:{}",std::mem::size_of_val(& *data));
print!(" ");

let mut handlers = vec![];

for i in 0..10{
        let cloned_data = Arc::clone(&data); // creates a new clone of the big size data for and gives to new thread.
    let handler = thread::spawn(
        move ||{
            println!("Size of cloned data is by thread {} is {} bytes",i+1,std::mem::size_of_val(& *cloned_data));
        }
    );

    handlers.push(handler);
}

for handler in handlers{
    handler.join().unwrap();
}

println!("Original Arc count:{}",Arc::strong_count(&data));
}