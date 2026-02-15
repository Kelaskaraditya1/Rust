/* Async and Await: 

Async and Await are similar to promises in react.

the brief meaning of async and await is that , the task which is taking indefinite time suspend it for some time and the worker tells when it is done so than rerun it.
 it returns a Feature type which is that the operation will be completed and it will return a value.
 so we have to use a run time for this , in which the operation runs.

 1) the first run time that we used is smol:
 it gives a smol::block_on(feature:Feature) which returns result of type <T>.

 for installing it use cargo add smol
 and for using it: smol::block_on(feature:Feature)

    now for running multiple tasks simultaniously we use the feature block.
    install the dependency : cargo add futures.

    if there are multiple feature functions than use the join!(f1,f2,f3,...) in a async block in block_on so that it returns a tuple of it's results.
    for using join!(), import  use futures::join;

        let result = smol::block_on(
        async{
            join!(num1,num2,num3)
        }
    );

    it is similar to num1.await, num2,await, num3,await .
    it returns a tuple of their results.
--------------------------------------------------------------------

SELECT! MACRO (futures crate)

The select! macro is used when we want to execute a block of code
as soon as one of multiple futures completes.

It waits for the FIRST future that becomes ready.

It is conceptually similar to:
- Promise.race() in JavaScript
- select in Go
- match statement in Rust (pattern-based branching)

But it works with asynchronous futures.

--------------------------------------------------------------------

WHEN TO USE select!

Use select! when:

1) Multiple async tasks are running.
2) You want to react immediately when ANY one finishes.
3) You do NOT want to wait for all tasks (unlike join!).

--------------------------------------------------------------------

BASIC STRUCTURE
async{
    loop {
    select! {
        value = future1 => { /* handle future1 */ },
        value = future2 => { /* handle future2 */ },
        complete => { break; }
        }
    }
}


--------------------------------------------------------------------

WHY select! IS USED INSIDE A LOOP

select! handles only ONE completed future per call.

If we want to:
- Continuously monitor multiple futures
- Process each as it completes

We wrap select! inside a loop.

Without loop → only the first completed future is handled.

--------------------------------------------------------------------

WHY .fuse() IS REQUIRED

Normal Future rule:

Once a future returns:
    Poll::Ready

It MUST NOT be polled again.

But inside a loop, select! may poll the same future again.

To prevent panic or undefined behavior, we convert it into a FusedFuture.

Example:

use futures::future::FutureExt;

let future = my_future().fuse();

--------------------------------------------------------------------

WHAT .fuse() DOES

.fuse() converts a normal Future into a FusedFuture.

After it completes:
If polled again → it safely returns:
    Poll::Pending

Instead of panicking.

So:

Normal Future:
    Pending → Ready → ❌ invalid to poll again

FusedFuture:
    Pending → Ready → Pending → Pending → ...

--------------------------------------------------------------------

WHY pin_mut! IS REQUIRED

select! requires:
    Pin<&mut Future>

Because futures must not move in memory after being polled.

So we use:

use futures::pin_mut;

pin_mut!(future1, future2);

IMPORTANT:
pin_mut! DOES NOT fuse.
It only pins.

--------------------------------------------------------------------

SPECIAL BRANCH: complete =>

Inside select!, we can use:

complete => { ... }

This branch runs when:
All fused futures have completed.

Useful to safely exit the loop.


join!() is used when we have to wait for all tasks to get over.
select!{} is used when on complition of 1 task something has to be done.

if we are not using any of this , and there is a async function than while calling it we have use await function.

async fn method_name(){

implementation..
}

method_name().await; // while using.



 

*/

use std::time::Duration;
use futures::join;
use futures::future::FutureExt;
use futures::select;
use futures::pin_mut;


fn main(){

    let number = 1331;
    let status = smol::block_on(check_palindrome(number));

    if status{
        println!("Palindrome");
    }else{
        println!("Not a Palindrome");
    }

    let num1 = num1().fuse();
    let num2 = num2().fuse();
    let num3 = num3().fuse();

    pin_mut!(num1, num2, num3); // fuses the Features.

    // let result = smol::block_on(
    //     async{
    //         join!(num1,num2,num3)
    //     }
    // );

    // println!("result:{:?}",result);

    let result = smol::block_on(
        async{
            loop{
            select!{

                x=num1=>println!("Num1 is completed, value is: {}",x),
                x=num2=>println!("Num2 is completed, value is: {}",x),
                x=num3=>println!("Num3 is completed, value is: {}",x),
                complete=>{
                    println!("All operations are completed");
                    break;
                    }
                }
            }
        }
    );




}

async fn check_palindrome(number:i32)->bool{

    let mut original_number = number;
    let mut palindrome_number = 0;

    while original_number!=0{
        palindrome_number=(original_number%10)+(palindrome_number*10);
        original_number/=10;
    }

    return palindrome_number==number;

}

async fn num1()->i32{

    std::thread::sleep(Duration::from_secs(1));
    10
}

async fn num2()->i32{

    std::thread::sleep(Duration::from_secs(1));
    15
}

async fn num3()->i32{

    std::thread::sleep(Duration::from_secs(1));
    20
}



