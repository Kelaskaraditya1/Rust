use std::io;
fn main(){

    /* String
    1) for pushing a string in another string: str.push_str("...")
    2) for pushing a charecter in another string: str.push('char')
    3) for concatinating 2 string s1 and s2 

    s3 = s1 + &s2 , which means we add the second string s2 to first string s1 and give the ownership of the result to s3.
    after this s1 does not remain invalid since the ownership is being transferred and s2 remains valid since we are passing reference of second string 
    it internally calls add(self,string:&String) 

    for multiple we can do : sn = s1 + &s2 + &s3 + &s4 + .... + &sn; 

     */

     let name = "Aditya"; // this is a string which is not owned and this can be converted to proper String by using to_string();
    let mut namaste = String::from("नमस्ते");
    println!("{namaste}",);

    namaste.push_str("in hindi");
    namaste.push('.');

    // For concatinating 2 string

    let mut string1 = String::from("Aditya");
    let mut string2 = String::from("Aditya Kekaskar");


    // let mut name = string1 + &string2;
    // println!("My name is :{name}");

    // for iteratinng over a string:

    // 1) charecter wise:

    // for ch in string1.chars(){
    //     print!("{}",ch);
    // }

    // 2) iterating using index and value:

    // for (index,char) in string1.char_indices(){
    //     println!("Index: {index} value: {char}");
    // }

// Important String functions:


/*

    1) length of string: string1.len()  
    2) upper and lower case: string1.to_uppercase(), string1.to_lowercase() 
    3) for removing unnecessary charecters: string1.trim()
    4) for finding whether one string contains another or pattern : string1.contain(&string2)
    

 */

 if string2.contains(&string1){
    println!("String 2 contains string 1");
 }else{
    println!("String is absent");
 }

// println!("{}",get_first_word_length(&string2));

/*
    Slices: slices are non owned ,imutable reference string which borrows some part of the string
    so while passing it as a function parameter we pass it as a reference and as a return type as well a reference , and use it similar to a normal string.

    when the starting and ending index is known: return &string[start..end];
    when only starting index is known: return &string[start..];
    when only ending index is known: return &string[..end];
    when no index is known and the entire string is requires: return &string[..];

    we can also use get(start..end); for getting the slice of the string. 

 */

 let stdin = io::stdin();
 let mut raw_string = String::new();

 stdin.read_line(&mut raw_string)
    .expect("Enter proper string");

println!("{}",get_last_word(&raw_string));




}

fn get_first_word_length(string:&String)->&str{

    for (index,char) in string.char_indices(){
        if char==' '{
            return &string[0..index];
        }

    }
    return &string[..];

}

fn get_last_word(string:&String)->&str{

for (index,char) in string.char_indices().rev(){
    if char==' '{
        return &string[index+1..];
    }
}
    return &string[..];

}