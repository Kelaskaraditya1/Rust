/* Serde stands for Serialization and De-serialization for converting a data to json. and json data to appropriate rust data format 

dependencies:

1) cargo add serde
2) cargo add serde --features derive
3) cargo add serde_json

1) Serialization : to convert any rust data to Json data.
steps: 
    1) add use serde::{Deserialize, Serialize}; use serde_json::to_string; imports

    2) than create structs as per requirements. and add annotation to struct 
    #[derive(Serialize,Deserialize)] // to implement serialization and deserialization
    #[serde(rename_all="camelCase")] // to convert the data in Camel Case.

    3) to convert data to Json format use : to_string(& object) it get Option<Ok,Err>

    so use let match case

    if result.is_ok(){
    data = result.ok().unwrap();
    }else{
    println!("Err :{:#?}",result.err());
    }

    to_string() prints in the result in one line. to print it in proper object format we have to use to_string_pretty();

2) Deserialization: to convert Json data to Rust data.

    let json_data = r#"
    {
    "key":"value"
        }
            "#;

    let object = from_str<T>(json_data); // T is the Type in which the data has to be converted.

    if object.is_ok(){
        object.ok().unwrap();
    }else{
        object.err();
    }





*/


use serde::{Deserialize, Serialize};
use serde_json::to_string_pretty;
use serde_json::from_str;

#[derive(Serialize,Deserialize,Debug)]
#[serde(rename_all="camelCase")]
struct User{

    name:String,
    contact:String,
    email:String,
    username:String,
    password:String,
    created_at:i32,
    address:Address

}

#[derive(Serialize,Deserialize,Debug)]
#[serde(rename_all="camelCase")]
struct Address{
    line1:String,
    line2:String,
    pin_code:String
}

fn main(){

    let address =Address{
        line1:"2/2 shivneri sadan, Patkar cmpd, Tulshet pada,".to_string(),
        line2:"Gav Devi road, Bhandup (W), Mumbai-400078".to_string(),
        pin_code:"400078".to_string()
    };

    let user1 = User{
        name:"Aditya Kelaskar".to_string(),
        contact:"8591059220".to_string(),
        email:"kelaskaraditya1@gmail.com".to_string(),
        username:"kelaskaraditya1".to_string(),
        password:"Aditya@1234".to_string(),
        created_at:32,
        address:address
    };

    // let ser_obj = to_string_pretty(&user1);

    // if ser_obj.is_ok(){
    //     println!("{}",ser_obj.ok().unwrap());
    // }else{
    //     println!("{:#?}",ser_obj.err());
    // }

    let json_data=r#"
    {
  "name": "Aditya Kelaskar",
  "contact": "8591059220",
  "email": "kelaskaraditya1@gmail.com",
  "username": "kelaskaraditya1",
  "password": "Aditya@1234",
  "createdAt": 32,
  "address": {
    "line1": "2/2 shivneri sadan, Patkar cmpd, Tulshet pada,",
    "line2": "Gav Devi road, Bhandup (W), Mumbai-400078",
    "pinCode": "400078"
  }
}
    "#;

    let user = from_str::<User>(json_data);

    if user.is_ok(){
        println!("{:#?}",user.ok().unwrap());
    }else{
        println!("{:#?}",user.err());
    }
    
}