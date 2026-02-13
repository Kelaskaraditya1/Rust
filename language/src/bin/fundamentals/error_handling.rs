/*
 Error Handling in rust is not done using try-catch block , we use Result<T,E> as a return type 
 this Result type has 2 varients Ok(T) and Err(E).

 where when a operation is done sucessfully it returns Ok(value) and the T is the type of the value.
 and when an error occurs we return Err(E) and E is the type of the error.

 this is used when we donot want to crash the programme and gracefully want to end the programmer without any crash.
 if we want the programme to crash at a certain condition than we can use panic!(message) macro which crashes the programme.

 */
use std::fs::File;
fn main(){
    println!("Error Handling");

    let result = match division(10, 0){
        Ok(value)=> value,
        Err(message)=>{
            println!("{message}");
            -1
        },
    };

    println!("Result: {result}");

    let file_open_status = File::open("hello.txt");

    let file = match file_open_status {

        Ok(file)=>{
            println!("Opening the file");
            file
        },
        Err(error)=> match error.kind(){
            std::io::ErrorKind::NotFound=>{
                match File::create("hello.txt"){
                    Ok(file)=>{
                        print!("file created sucessfully");
                        file
                    },
                    Err(e)=>panic!("failed to create the file:{e:?}"),
                }
            },
            _=>panic!("Some thing went wrong!!{error:?}"),        
        }  
        
    };



    
}

fn division(a:i32,b:i32) -> Result<i32,String>{

    if b==0{
        return Err(String::from("cannot divide by 0"));
    }

    Ok(a/b)
}