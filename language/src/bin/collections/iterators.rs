/*

Iterators: iterator are special functions that does some job

iter() returns a Iterator which is used to iteration over a collection 

1) sum(): it returns sum of a collection but it requires a iterator
for eg: 
    let collection = [...];
    collection.iter().sum();

2) map(): if we want to mutate every element while iterating it 
    map function takes an iterator and after performing operation it returns another iterator which will help for iterating.
    map function takes a closure/anonymous function as a parameter.

3) filter(): if we want to filter elements on the basis of some condition
    filter function also takes a anonymous function which returns a boolean and if it returns true than it is innvolved else rejected.

4) we can use the map and filter function in chain as well. and to convert the iterator to vector use .collect()

*/

fn main(){

    let vector = vec![10,20,30,40,50,60,70,80,90,100,105,109,110];
    // let vector_iterator = vector.iter(); // returns iterator which is help for iterating.

    // for element in vector_iterator{
    //     println!("element:{element}");
    // }

    // 1) Sum function:
    // let sum:i32 = vector.iter().sum();
    // println!("Sum is {sum}");

    // 2) Map function:

    // let iterator = vector.iter();

    // let new_iterator = iterator.map(
    //     |x| x+1
    // );

    // for element in new_iterator{
    //     print!("{} ",element);
    // }

    // 3) Filter function:


    // let iterator = vector.iter();

    // let new_iterator = iterator.filter(
    //     |x| *x%2==0
    // );

    // for element in new_iterator{
    //     print!("{} ",element);
    // }

    // 4) chaining

    let updated_vector:Vec<i32> = vector.iter()
    .filter(
        |x| *x%2==0
    ).map(
        |x| x*2
    ).collect();

    println!("{:?}",updated_vector);

    








     
}