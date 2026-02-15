/*
Closures: closure are anonymous functions without any name and stored in a variable 

syntax: 

1)  full fledged syntax:

    let function = |a:type, b:type|-> type{
implementation...
result // returns result
};

2) if the implementation is a single line , and only a return statement than

    let function = |a:type, b:type| result 

for using : println!(function(a,b));

mentioning the type of the parameter is necessary , the type of return type is not necessary.

3) if the variable is in the same scope as the function than no need to take it as a parameter it will be taken from the scope

    let value = 10;

    let print_statement = || println!("{value}");

    print_statement();



*/

fn main(){

    let add = |a:i32, b:i32|->i32 {
        a+b
    };

    let subtraction = |a:i32, b:i32|  a-b;

    println!("Addition:{}",add(10,20));
    println!("Subtraction:{}",subtraction(20,10));

    let value = 10;

    let print_statement = || println!("{value}");

    print_statement();
}