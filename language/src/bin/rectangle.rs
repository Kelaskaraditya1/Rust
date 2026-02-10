use std::io;

/* struct is only for writing the fields which are required , for writing the methods which are related to the struct we have to create a impl <Struct name>.
the normal functions which are not the part of any struct implementation are called as functions and those who are part of struct implementation are called as methods.
for creating a function which can be accessed by an object , in the function defn the first paramenter should be &self always which points to the object , it is similar to this keyword.
for declaring a static method , the static method does not have a &self indicating it belongs to class and not a particular object and the return type is Self 

 */

fn main(){

    let mut length_input=String::new();
    let mut bredth_input= String::new();

    println!("Enter Length of the Rectangle");

    io::stdin().read_line(&mut length_input)
        .expect("Enter proper length");

    println!("Enter Bredth of the Rectangle");

    io::stdin().read_line(&mut bredth_input)
        .expect("Enter ptoper bredth");

    let length:i32 = length_input.trim()
        .parse()
        .expect("Length is not in proepr format");

    let bredth:i32 = bredth_input.trim()
        .parse()
        .expect("Bredth is not in proper format");

    let rectangle = Rectangle{
        length,bredth
    };

    let rectangle2 = Rectangle{
        length:30,
        bredth:30
    };

    if rectangle.validate(){
        println!("Area of the rectangle {:?} is {}",rectangle,rectangle.area());
        if rectangle.can_hold(&rectangle2){
            println!("Rectangle can hold another rectangle")
        }else{
            println!("Rectangle cannot hold another rectangle");
        }
    }

    let rectangle = Rectangle::create_rectangle(length, bredth);
    println!("The Rectangle from static method is {:?}",rectangle);

}

#[derive(Debug)]
struct Rectangle{
    length:i32,
    bredth:i32
}

impl Rectangle {

    fn validate(&self)->bool{
        if self.length<0{
            println!("Length cannot be negative");
            return false;
        }

        if self.bredth<0{
            println!("Bredth cannot be negative");
            return false;
        }

        true
    }

    fn area(&self) -> i32{
        self.length * self.bredth 
    }

    fn can_hold(&self, rectangle:&Rectangle)->bool{

        if self.length>=rectangle.length && self.bredth>=rectangle.bredth{
            return true;
        }

        return false;

    }

    fn create_rectangle(length:i32,bredth:i32)->Self{
        Rectangle { length, bredth }
    }
    
}

// fn area(rectangle:Rectangle)->u32{
//     rectangle.length*rectangle.bredth
// }