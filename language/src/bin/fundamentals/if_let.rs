/*  If Let statement, alternative to match case , match case is exhastive means we have to cover all the cases
but in if let statement we just have to cover the required cases not all and it is useful if there are only 2 cases.



*/
use std::io::{self};

fn main(){

    // let mut raw_num = String::new();
    // let mut raw_status = String::new();

    //         println!("Enter the first number");
            
    // io::stdin().read_line(&mut raw_num)
    //     .expect("Enter proper number");

    // io::stdin().read_line(&mut raw_status)
    //     .expect("Enter proper status");

    // let number:i32 = raw_num.trim()
    //     .parse()
    //     .expect("unable to convert to proper number");

    // let status:bool = raw_status.trim()
    //     .parse()
    //     .expect("unable to convert to proper status");

    let stdin = io::stdin();

    let mut raw_num = String::new();
    stdin.read_line(&mut raw_num)
        .expect("Enter proper number");

    let num:i32 = raw_num.trim()
        .parse()
        .expect("Numbe should be of inteegr format");

    if let Some(sum) = get_sum(num){
        println!("The sum is {}",sum);
    }else{
        println!("You entered zero, enter a number greater than 0");
    }
        


}

fn get_sum(num:i32)->Option<i32>{

    if num==0{
        None
    }else{
        Some(num+10)
    }

}

