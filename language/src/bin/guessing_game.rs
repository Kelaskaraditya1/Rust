/*

for taking custom input from user/console use std:io library , for importing we use "use" keyword
use std:io;

Prelude: a set of libraries which are already imported when a .rs fiel is created to reduce the complexicity
similar to "java.lang" package , no need to import it , it is already imported.

for reading the values from terminal:

1) use io::stdin() which is the standard definition for reading from terminal , this returns a instance of standard input , helps for taking input from terminal.
2) use the read_line() function of it. io::stdin().read_line()
3) read_line(<name of var>) , expects a variable in which the value coming from the terminal has to be appended , like it does not writes or overides the value
it appends the new value to the existing value.
4) the read_line(&mut var) expects a reference of a mutable var. and it returns a Result which is a enum type , Ok and Err 
5) we can also add a .expect("message"), if it fails taking the input , than it throws the error message in the console.

 */

 /* <================================================================================================Variables================================================================================================>

 1) By default the variable in rust are im mutable , once assigned their value cannot be changed.
 let a = 10, no need to use datatype , rust automatically identifies the type of the var

 to make it mutable , we have to use mut keyword
  */

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main(){
    println!("Guessing Game");

    let secret_number = rand::thread_rng().gen_range(1..=100); // Generating random number between 1-100
    println!("Secret number is:{}",secret_number);

    loop{
            let mut guess = String::new();
    println!("Enter a number between 1-100");

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to take the input");

    let guessed_number:u32 = match guess.trim().parse(){
        Ok(num)=>num,
        Err(_)=>{
            println!("Enter valid input");
            continue;
        },
    };

    // let guessed_number:u32 = guess.trim()
    //     .parse()
    //     .expect("Failed to parse the number"); // the read_line() gives a String , we convert it to uts32

    println!("Your guessed number is: {}", guessed_number);

    match guessed_number.cmp(&secret_number){
        Ordering::Equal=>{
            println!("You won!!");
            break;
        },
        Ordering::Greater=>println!("Too Big"),
        Ordering::Less=>println!("Too Small"),
        }

    }

    

}