/*
Shadowing:

shadowing is a concept in which we can make another variable with the  same name as earlier,
for eg let aditya = 32;
...
let aditya = 32;

 */

fn main(){

    let x = 5;

    let x = x+1;

    {
        let x = x*2;
        println!("Value of X in inner Scope: {x}");
    }

    println!("Value of X in outer Scope: {x}");

}