/*  The problem with below code is that , we give a word to the funcion and than it returns a index
but than once the index is returned than we have changes the string to a new string , but still the function gives the same index. 

So basically we want something like untill the index var is holding the result till than the name should not be mutated.
so in this we can use slice type, which returns a reference of substring of the string and the reference doesnot allow to mutate the original value before using it 

slice syntax:

let string = String::from("Aditya Kelaskar");
let slice1 = & string[start..end+1], since ending index is exclusive [start,end)
let slice = & string[..end] starting index = 0
let slice = & string[start..] ending index = last index
let slice = & string[..] , complete string  




*/

fn main(){

    let mut name = "Aditye Kelaskar";
    let index = get_first_word_length(&name);
    println!("The first word is: {index}");
    name = "SandeepKelaskar";
    


}

// fn first_word(string:String)->String{

//     for(index,char) in string.chars().enumerate(){
//         if char==' '{
//         let sub: String = string.chars().take(index).collect();
//             return sub;
//         }
//     }

//     string

// }

fn get_first_word_length(string: &str)->&str{

    let bits = string.as_bytes();

    for (index, &item) in bits.iter().enumerate(){

        if item ==  b' '{
            return & string[..index];
        }

    }

    return & string[..];
}