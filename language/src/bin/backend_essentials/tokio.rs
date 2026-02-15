/*  Tokio: Tokio is another more robust runtime/executor.

for adding the tokio runtime in the project 
    1) cargo add tokio.
    2) cargo add tokio --features=macros // for accessing annotaiton and macros that tokio provides.
    3) cargo add tokio --features=rt

    normal async await syntax:

async fn method_name(){
implementation..
}

method_name().await; // while using.

Feture: Future is similar to promise that is it will return a value when we poll it.
so it returns 2 values either Ready(value) and Pending
when the operation is not completed yet and we ask it , than it returns Pending and when the operation is completed it returns Ready with the value.


*/

use std::time::Duration;

struct Racer{

    name:String,
    completed_laps:u8,
    laps: u8,
    best_lap_time:u8,
    lap_times: Vec<u8>,

}

impl Racer{

    fn new(name:String, completed_laps:u8, laps:u8, best_lap_time:u8, lap_times:Vec<u8>)->Self{
        Racer{
            name,
            completed_laps,
            laps,
            best_lap_time,
            lap_times
        }
    }

    fn do_lap(&mut self){

        let lap_time = self.lap_times.pop();

        if lap_time.is_some() && self.best_lap_time>lap_time.unwrap(){
            self.best_lap_time=lap_time.unwrap();
        }

        self.completed_laps+=1;

    }

}

impl std::future::Future for Racer{

    type Output = u8;
    
    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        if self.completed_laps<self.laps{
            println!("{} is doing a lap",self.name);
            self.get_mut().do_lap();
            cx.waker().wake_by_ref();
            return std::task::Poll::Pending; 
        
        }

        print!("{} has done all the laps",self.name);
        return std::task::Poll::Ready(self.best_lap_time);
    }



}

#[tokio::main(flavor="current_thread")]
async fn main(){

    // print_tokio().await;

    let aditya = Racer{
        name:"Aditya Kelaskar".to_string(),
        completed_laps:0,
        laps:5,
        best_lap_time:200,
        lap_times:vec![100,90,80,70,60]
    };

    let best_time= aditya.await;
    print!("Best time laps:{best_time}");
}

// async fn print_tokio(){

//     std::thread::sleep(Duration::from_secs(2));
//     println!("Tokio RunTime");

// }