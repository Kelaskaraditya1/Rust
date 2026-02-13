/*
Functions:

rules: 
    1) name of the function should be in snake case , means "_" seprated
    2) the parameter should have valid datatypes , for variables if we donot mentioin than it's fine, but for functions it is necessary.
    

generatl syntax:

    fn name_of_function(list of params) -> return type {
    .. implementation ..
    }

    In rust generally the last statement which is without a ';' is considered a return statement, and in this case there is no need of adding  a ;  

 */

 /* Statements and Expression:

 Expression is which reutrns a value and does not contains a ';'

 Statement is something which consumes the value given by Expression and contains a ';' and does not returns any value.

 for eg: let value = 20+20;

 so 20+20 = 40 -> Expression since it gives a value 
 and let value = 20+20; this complete is a statement since it doesnot returns any value and contaisn a ';'
 
  */

//  fn print_some_thing(){

//     println!("I am Vengence, I am the Knight, I am Batman");
//  }

//  fn add_one_and_print(x:i32){

//     println!("The value after adding 1 is {}",x+1);
//  }

 fn expression_example() -> i32{
   let x = {
      let y=10;
      y+10  // return statement
   };

   return x+10
 }

fn main(){
   let value = expression_example();
   println!("The value is:{value}");
}