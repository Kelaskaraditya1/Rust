/*  References

References is the solution for ownership.
In case of function call the the variable which is passed as a parameter cannot be used again since it's ownership is expired with the function statement
so to solve this issue we use references in which we pass the Address of the variable in the parameter, but in function defn no need to de reference , we can perform all the operations normally
as if it is a normal variable.
and by this the issue of ownership is also solved , and we can reuse the variable which is passed as a paramenter again after function call.

Dangling reference:

when we try to return a reference to an object as a return type of a variable whose scope is over in the function defenition than it gives error.
since the scope of the variable ends with the function , so returning a reference of a variable could point to null.


*/

fn main(){

    let name = String::from("Aditya");

    length_of_string(&name);

    let string2 = String::from("Kelaskar");
    println!("My name is {name}");

}

fn length_of_string(string:& String)->usize{
    // string.push_str(string);
    let length = string.len();
    length
}