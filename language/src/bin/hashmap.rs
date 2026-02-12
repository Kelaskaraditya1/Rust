/* Hashmap is a list of key, value pair 

for using hashmap we have to use , use std::collections::HashMap;

1) Initialization of an empty hashmap: let hashmap:HashMap<type,type> = HashMap::new();
2) for inserting value in map: map.insert(key,value);
3) for getting value from map: map.get(&key) , we have to pass reference of the key.
4) for checking whether a key is present in the map or not: map.contains_key(key)
5) if the key already exists than the value is overriden.
6) for removing a pair of key and value: map.remove(key).
7) for getting the length of the map: map.len().

*/

use std::collections::HashMap;
use std::io;
fn main(){
    let mut hashmap:HashMap<String,i32> = HashMap::new();
    let stdin = io::stdin();

    // hashmap.insert(String::from("Aditya"),1);
    // hashmap.insert(String::from("Sandesh"), 2);
    // hashmap.insert(String::from("Ganesh"), 3);

    // println!("{}",hashmap.get(&String::from("Adi")).unwrap_or(&-1));

    // let value= match hashmap.get(&String::from("Aditya")){
    //     Some(value)=>value,
    //     None=>{
    //         println!("Key does not exist");
    //         &-1
    //     }
    // };

    // iterating the map

    //1) iterating with key and value together

    // for (key,value) in hashmap{
    //     println!("{key} => {value}");
    // }

    // iterating over list of keys.

    // for key in hashmap.keys(){
    //     println!("Key:{key}");
    // }

    // iterating over list of values.

    // for value in hashmap.values(){
    //     println!("Value: {value}");
    // }

    // Mutable iteration

    // for value in hashmap.values_mut(){
    //     *value+=10;
    // }

    // println!("{:?}",hashmap);

    // for (index,value) in hashmap.iter_mut(){
    //     *value+=10;
    // }

    // println!("{:?}",hashmap);

    // println!("Enter the number of elements");
    //  let mut raw_size = String::new();

    //  stdin.read_line(&mut raw_size)
    //     .expect("Enter proper size");

    // let size:usize = raw_size.trim()
    //     .parse()
    //     .expect("cannot convert to proper usize");

    // for i in 0..size{

    //     println!("Enter the key");
    //     let mut  raw_key = String::new();

    //     stdin.read_line(&mut raw_key)
    //         .expect("Enter proper key");

    //     let mut raw_value = String::new();
    //     stdin.read_line(&mut raw_value)
    //         .expect("Enter proepr value");

    //     let value:i32 = raw_value.trim()
    //         .parse()
    //         .expect("cannot convert to i32");

    //     hashmap.insert(raw_key, value);

    // }

    // println!("{:#?}",hashmap);

    // Hashmap which maintains the word and its count

    let sentence = "hello world hello world Aditya Aditya Kelaskar Sandeep";
    let mut word_counts:HashMap<String,i32> = HashMap::new();

    for word in sentence.split_whitespace(){
        let count = word_counts.entry(word.to_string()).or_insert(0);
        *count+=1;
    }

    for (key,value) in word_counts{
        println!("{key} => {value}");
    }





}