use std::io;

struct User{
    name:String,
    contact:String,
    email:String,
    username:String,
    password:String
}

fn main(){

    // let mut aditya = User{
    //     name:String::from("Aditya"),
    //     contact:String::from("8591059220"),
    //     email:String::from("kelaskaraditya1@gmail.com"),
    //     username:String::from("kelaskaraditya1"),
    //     password:String::from("Aditya@1234")
    // };

    // let sandesh = form_user(String::from("Sandesh"),
    //     String::from("8591059220"),
    //         String::from("sandy1@gmail.com"), 
    //         String::from("sandy1"), 
    //         String::from("Sandesh@1234")
    //     );

        // println!("Name of the user {}",sandesh.name);

    // let username = aditya.username;

    let mut length_input=String::new();
    let mut bredth_input = String::new();

    println!("Enter the Length");

    io::stdin()
        .read_line(&mut length_input)
        .expect("Enter proper length");

    println!("Enter the Bredth");

    io::stdin()
        .read_line(&mut bredth_input)
        .expect("Enter proper bredth");

    let length:u32 = length_input.trim()
        .parse()
        .expect("Enter proper length");

    let bredth:u32 = bredth_input.trim()
        .parse()
        .expect("Enter proper bredth");

    println!("Length = {length} Bredth = {bredth}");



    



}

// fn form_user(name:String, contact:String, email:String, username:String, password:String)->User{
//     User { name, contact, email, username, password }
// }