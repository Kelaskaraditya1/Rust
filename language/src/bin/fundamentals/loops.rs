        /*

        In rust for using loop we have use loop{} keyword.

        but this is a while loop and a infinite loop , so we have to specify the condition for stopping and also the condition for increment and decrement. 

        we can use loops for returning the value as an expression as well, like once all iterations of the loop are completed than it gives a value 
        or if we want to break at a certain condition than we can simply break without any value or we can reutrn a value as well

        if condition{
        break value;
        }

        we can also give name to loops in case of nested loops : use "'"+name as name of the loop

        'name: loop{
        
            loop{
        
        
            }
        
        }

        and we can break the outer loop with the help of name , break 'name ; 


        


         */

        /*

        While Loop:

        when the condition is already known than we use the while loop.

        while condition{

        updation

        }
        
         */


fn main(){

        // loop{   // runs for infinite time
        //     println!("I am Ironman");
        // }

        // let mut x = 0;

        // let value= loop{
        //     if x>=5{
        //         break x*2;
        //     }

        //     println!("I am Aditya {x}");
        //     x=x+1;
        // };

        // println!("value of x is {value}");

        // let mut count = 0;

        // 'outer:loop{
        //     println!("count = {count}");

        //     let mut remainning = 10;

        //     loop{

        //         println!("remainning = {remainning}");
        //         if remainning == 9{
        //             break;
        //         }
        //         if count==2{
        //             break 'outer;
        //         }

        //         remainning-=1;
        //     }
        //     count += 1;
        // }

        // let mut count = 0;

        // while count!=10{
        //     println!("Count is {count}");
        //     count+=1;
        // }

        // For iterating over an Array:

        // let array = ["Aditya", "Sandesh", "Parth", "Jivesh", "Rohan", "Varad", "Ganesh"];

        // for element in array{
        //     println!("Name :{element}");
        // }

        // Normal For Loop

        // for x in 0..10{
        //     println!("Value of x is {x}");
        // }



}