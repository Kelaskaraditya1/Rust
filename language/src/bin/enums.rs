/*  Enums , when there are fixed objects of a class in that case we create enums 
enums can also store other types , tuple, String
we also can have auxillary methods for enums , similar to struct and they are written in the same way as for struct.

according to match case all case should be covered , not written so for special cases we can write down the cases and for cases the output the same we can use the variable other or _
and if we donot want to do anything we can return () empty tuple.


Option<Type> , it is used when the value is optional to pass.
it has 2 parts Some(value) which contains the value and None which means no value 

and we can use match for getting value if Some(value)=> value , None=>default value.

and for getting value we have to use unwrap() or unwrap_or(default value).



*/


use std::io;

#[derive(Debug)]
enum Type{
    V4(u8,u8,u8,u8),
    V6(String)
}

// #[derive(Debug)]
// struct IpAddress{
//     address:String,
//     ip_type:Type
// }


// impl IpAddress {

//     fn new_ip(address:String)->Self{
//         IpAddress {
//             address:address,
//             ip_type:Type::V4
//          }
//     }
    
// }

fn main(){

    // let mut raw_address = String::new();

    // println!("Enter address");
    // io::stdin().read_line(&mut raw_address)
    //     .expect("Enter proper ip ");

    // let address1 = Type::V4(192,168,0,1);
    // let address2 = Type::V6(String::from("192.168.0.1"));

    // route(address1);
    // route(address2);

    //     print!("Enter status");

    // let mut raw_status = String::new();

    // io::stdin().read_line(&mut raw_status)
    //     .expect("Enter proper status");

    // let status:bool = raw_status.trim()
    //     .parse()
    //     .expect("cannot convert to boolean value");

    // let sum = 20+get_value(status).unwrap_or(50);
    // println!("Sum is :{sum}");

    let mut raw_step = String::new();

    io::stdin().read_line(&mut raw_step)
        .expect("Enter proper step");

    let step:i32 = raw_step.trim()
        .parse()
        .expect("Not a valid step");

    die_roll(step);



}

fn route(ip_address:Type){
    println!("Routing request to: {:?}",ip_address);
}

fn get_value(status:bool)->Option<i32>{

    if status{
        return Option::Some(30);
    }else{
        return Option::None;
    }

}

fn die_roll(step:i32){

    match step{
        3=> println!("You have got an hat"),
        6=> println!("You have lost your hat"),
        order=> println!("you moved ahead {order} steps"),
    }

}