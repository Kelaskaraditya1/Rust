/*

Compound Datatypes & Scalar Datatypes:

Scalar Datatypes: scalar datatypes are basic and fundamental types , Integer, Floating, Char, String, Boolean

Compound Datatypes: which are arrays and tuples

    1) Tuple : tuple is list of fixed size , and hetrogenous items. and use "()".
    for eg let tup1 = ("Aditya", true, 100);

    for accessing the values of the tuple: 
        1) using index of the tuple (0 based Indexing): tup1.0, tup1.1
        2) destructuring the tuple: let (x,y,z) = tup1;
            println!("X:{x} , Y:{y}, Z:{z}");

    an empty tuple is an called as Unit ()


    2) Arrays: arrays is list of fixed length and homogenous elements , and uses "[]" for entering the element
                for creating a array of same elements for n number of times use [element;count]
                    for eg let a = [10;5] =>[10,10,10,10,10]

    
 */

fn main(){

    // Tuples

    // let details = ("Aditya", 8.86, true);

    // let (name, pointer, pass) = details;

    // println!("Name of the Student is: {name}");
    // println!("Pointer of the Students is: {pointer}");

    // if(pass){
    //     println!("Student has passed");
    // }else{
    //     println!("Student has failed");
    // }
        
    // Arrays

    let array = [20,30,40,50,60,70];

    for item in &array {
    println!("Item: {}", item);
}

    let array1 = [10;5];

    for item in &array1 {
    println!("Item: {}", item);
}

    



}