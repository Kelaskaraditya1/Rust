/* Threading in Rust:

for creating a new thread: 

1) impor: use std::thread;
2) use the spawn function to create a new thread, it expects single arguement which is closure which tells what task to perform.
spawn method is non blocking means the main thread will create a new thread and contnue the further execution without waiting for the new thread to complete its work.
3) a new thread is created and it contains a JoinHandle which contains information of the thread and it has 2 methods is_finished() and join().
is_finished() method is used to check if the thread has completed its work or not returns a boolean value.
join() method is used to block the calling thread usually (main) and wait for the worker thread to complete it's execution and return the result , it either returns the result of the thread or an error if the thread panicked.

*/

use std::{thread::{self,}, time::Duration};

fn main(){
    println!("Threading in Rust");

    println!("t0: Main thread started working");

    // Defining the task for thread.

    let thread_task = ||{ 
        println!("t0: Thread1 started");
        for i in 0..=5{
            println!("Task completed by Thread1:{}",i);
            thread::sleep(Duration::from_secs(1));
        }
        println!("t0:Thread1 finished");

        69
    };

    // Spawning/creating a new thread.

    let handler = thread::spawn(thread_task);

    println!("Main thread continues execution");

    for i in 0..=5{
        println!("Task completed by main thread:{}",i);
        thread::sleep(Duration::from_secs(1));
    }

    // checking if Thread 1 has completed the execution or not

    if handler.is_finished(){
        println!("Thread1 has completed the execution");
    }else{
        println!("Thread1 is still in execution");
    }

    // Joining the thread i.e blocking the main thread untill the execution of thread1 is not completed.

    println!("Result of execution of thread1 is:{}",handler.join().unwrap());




}

