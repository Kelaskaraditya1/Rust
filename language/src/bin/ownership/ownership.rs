/*  Ownership

every object has a owner, and the value of the obect exists only till the owner exists.

so basically the variable belongs in a scope and when the scope is over than rust internally calls a drop function , which returns the memory to the system.

*/

/*  Ownership issue:
    let s1 = String::from("Aditya");
    let s2:String = s1;

    println!("{s1}");

    in this code it will now work and will give error.

    in String there are 3 components , reference to heap object, length of String, capacity 
    so s2 and s1 references to same object

    so when the scope of s1 is over than it's object is removed but s2 is also pointing to same object so it is pointing to null

    and than again when the scope of s2 is over than it will also try to remove an empty object.so this is the issue 

    for solving this issue we can use the clone method: s2 = s1.clone()

    So this issue also comes in when function call , when a var is passed to function and than reused the variable than it will give error of borrowing.


*/

fn main(){

    let name = String::from("Optimus Prime");
    take_ownership(name);

    let s1 = give_ownership();
    println!("The value of give_ownership {s1}");

    let s2 = give_and_take_ownership(s1);

    length_of_string(s2);




}

fn take_ownership(name:String){
    println!("Greetings, I am {name}");
}

fn give_ownership()->String{
    let value = String::from("I am Ironman");
    value
}

fn give_and_take_ownership(value:String)->String{
    value
}

fn length_of_string(string:String)->usize{
    let length:usize= string.len();
    length
}
