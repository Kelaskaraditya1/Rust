/*  Generics: in Java the use of generics is to pass a arguement of a specific type for a datastructure or in a function parameter.

in rust to reduce the repeation of the logic we use generic , for eg one logic is same for other types than we use generics for it so that the logic can be used for other types as well.

using generics in function: it accepts the arguement of the type T and it's return type is also T.

fn name<T>(parameter:<T>)->T{
}

*/

struct Point<U,V>{
    x:U,
    y:V
}

impl <U,V> Point<U,V> {

    fn new(x:U, y:V)->Self{
        Self { x, y }
    }
    
}

/*  Generics in struct

if we want the struct paramenter to have a particular type than while making an object of the struct we can accept it as paramenter.
for creating struct of which will accept parameter of particular type.

struct name<T1, T2, T3, ...>{
parameter1: T1,
parameter2: T2,
...
}

and for implementation block.

impl <T1, T2, T3, T4, ...> name<T1, T2, T3, T4,...>{

fn name(p1:T1, p2:T2, ...)->T{
...
}

}

*/

fn main(){
    println!("Generics");

    let vector1 = vec![110,100,60,40,70,50,60];
    let vector2 = vec![10.2,11.6,55.6,54.6,89.9];

    let list1 = [10,30,100,80,90,110,60];

    println!("Largest element in the vector:{}",largest_number(&list1));
    println!("Largest element in float vector: {}",largest_number(&vector2));
}

fn largest_number<T:PartialOrd>(vector:&[T])->&T{

    let mut largest = vector.get(0).unwrap();

    for element in vector{
        if element>largest{
            largest=element;
        }
    }

    return largest;



}