/*  If Let statement, alternative to match case , match case is exhastive means we have to cover all the cases
but in if let statement we just have to cover the required cases not all and it is useful if there are only 2 cases.



*/
use std::io::{self, Read};

fn main(){

    let mut raw_num = String::new();
    let mut raw_status = String::new();

            println!("Enter the first number");
            
    io::stdin().read_line(&mut raw_num)
        .expect("Enter proper number");

    io::stdin().read_line(&mut raw_status)
        .expect("Enter proper status");

    let number:i32 = raw_num.trim()
        .parse()
        .expect("unable to convert to proper number");

    let status:bool = raw_status.trim()
        .parse()
        .expect("unable to convert to proper status");

    println!("The sum of both the numbers is : {}", get_sum(number, status).unwrap());
}

fn get_sum(num1:i32, status:bool)->Option<i32>{
    if status{
        Some(num1+50)
    }else{
        Some(num1)
    }
}