use std::collections::HashMap;

#[allow(dead_code,unused_variables)]

// fn sort_string(string:&String)-> String{

//     let mut string_vec:Vec<char> = string.chars().collect();
//     string_vec.sort_unstable();

//     return string_vec.into_iter().collect();
// }

// fn valid_anagram(string1:String, string2:String)->bool{

//     if string1.len()!=string2.len(){
//         return false;
//     }

//     let str1 = sort_string(&string1);
//     let str2 = sort_string(&string2);

//     if str1.eq(&str2){
//         return true;
//     }

//     return false;

// }

// fn two_sum(vector:Vec<usize>, target:usize){

//     // Approach 1: using hashmap

//     let mut hashmap:HashMap<usize,usize> = HashMap::new();

//     for (index,value) in vector.iter().enumerate(){

//         hashmap.insert(*value, index);

//         if hashmap.contains_key(&(target-value)){
//             let first_index = hashmap.get(&(target-value)).unwrap();
//             print!("Pair is:{first_index} and {index}");
//             return;
//         }

//     }
//     println!("No pair found");

// }

fn reverse_integer(number:usize){

    let mut reverse = 0;
    let mut org_number= number;

    while org_number!=0{
        reverse = (reverse*10) + org_number%10;
        org_number/=10;
    }

    println!("The reverse number is:{reverse}");

}

fn missing_number(mut vector:Vec<usize>)->usize{
    vector.sort();

    let sum1:usize = vector.iter().sum();
    let sum2 = (vector.len()*(vector.len()+1))/2;

    sum2-sum1
}

fn main(){
    println!("Practise questions");

    // let string1 = String::from("aditya");
    // let string2 = String::from("aytidaaada");

    // if valid_anagram(string1, string2){
    //     print!("Valid Anagram");
    // }else{
    //     println!("Invalid Anagram");
    // }

    // let nums = [2,7,11,15];
    // two_sum(nums.to_vec(), 11);

    // reverse_integer(1234);

    let mut vec = [0,2,4,1,6,5];
    println!("Missing number:{}",missing_number(vec.to_vec()));

    


}