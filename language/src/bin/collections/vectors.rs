/* Vectors: vectors are growable and homogenous arrays.

1) for defining an empty vector , let vec = Vec::new(), it takes the type of the first value that we enter.
2) for defining an array with predefined elements, let vec = vec![1,2,3,4,5,...];
3) for getting value at a particular index, &vec[index] or vec.get(index) , get returns a enum of value Some and None so we can use match case if no value is present 
or index out of bound.and if want to return -ve number return as &-value

*/

use std::io::{self, Read};

#[derive(Debug)]
enum ELement{

    INT(i32),
    FLOAT(f64),
    TEXT(String)
    
}

fn main(){
    println!("Vector");

    // let mut vector = Vec::new();
    // let mut raw_size = String::new();
    let stdin = io::stdin();
    // vec.push(10);
    // vec.push(20);
    // vec.push(30);
    // vec.push(40);
    // vec.push(50);

    // let vec2 = vec![1,2,3,4,5];

    // println!("{:?}",vec);
    // println!("{:?}",vec2);

    // vector with a custom user input.

    // println!("Enter the size of the vector");

    // stdin.read_line(&mut raw_size)
    //     .expect("Enter proper size");

    // let size:i32 = raw_size.trim()
    //     .parse()
    //     .expect("cannot convert to integer");

    // for i in 0..size{
    //     let mut raw_input = String::new();

    //     println!("Enter value");
    //     stdin.read_line(&mut raw_input)
    //         .expect("Enter proper value");

    //     let mut input:i32 = raw_input.trim().parse()
    //         .expect("cannot convert to integer");
    //     vector.push(input);
    // }

    // println!("vector: {:?}",vector);

    // for getting a particular value from vector use &vec[index] or vec.get(index)

    // let mut raw_index = String::new();
    // println!("Enter the index of which the value has to be obtained");

    // stdin.read_line(&mut raw_index)
    //     .expect("Enter proper index");

    // let index:usize = raw_index.trim()
    //     .parse()
    //     .expect("cannot convert to integer value");

    // let value = match vector.get(index) {
    //     Some(value)=> value,
    //     None=> {
    //         println!("Index out of bound please enter valid index");
    //         &-1
    //     }
    // };

    // if *value!=-1{
    //     println!("Value at index {} is {}",index,value);
    // }else{
    //     println!("Index out of bound please enter valid index");
        
    // }

        // for iterating the vector.

    // for value in vector{
    //     println!("{}",(value)*2)
    // }

    // for forming a vector of hetrogenous types by defining a enum.

    let mut vector2 = Vec::new();

    vector2.push(ELement::INT(10));
    vector2.push(ELement::FLOAT(3.142));
    vector2.push(ELement::TEXT(String::from("I am Ironman")));
    vector2.push(ELement::INT(20));
    vector2.push(ELement::FLOAT(9.8));
    vector2.push(ELement::TEXT(String::from("I am Vengence, I am the Knight, I am Batman!!")));

    // for value in vector2{
    //     println!("{:?}",value);
    // }

    let mut raw_index = String::new();

    stdin.read_line(&mut raw_index)
        .expect("Enter proper index");

    let index:usize = raw_index.trim()
        .parse()
        .expect("cannot convert to i32");

    match vector2.get(index) {

        Some(ELement::INT(value))=>println!("The value is {value}"),
        Some(value)=>println!("The value is {:?}",value),
        None=> println!("Enter valid index"),

    }




}