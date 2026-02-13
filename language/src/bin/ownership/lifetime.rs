/*  Lifetime is used to solve dangling reference.

dangling reference is a problem in which something points to a memory location which is already been cleaned.

*/

// fn main(){
//     // let result;

//     // {
//     //     let y = 10;
//     //     result = &y;
//     // }

//     // println!("result",result);

//     let string1 = String::from("Aditya");
//         let string2 = String::from("Aditya Kelaskar");
//         let longest = longest_string(&string1, &string2);
//     println!("Longest string:",longest);

// }

// fn longest_string(s1:&String, s2:&String)->&str{
//     if s1.len()>s2.len(){
//         return s1;
//     }

//     s2;
// }

fn main(){
    
}