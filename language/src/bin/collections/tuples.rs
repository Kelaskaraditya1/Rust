/*  TUPLE IN RUST

Tuple is a collection of values of different types.

Key Points:

1) Fixed size.
2) Can store different data types.
3) Stored on stack (usually).
4) Order matters.
5) Size is known at compile time.

-----------------------------------------------------

1) Initialization:

let tuple = (10, 20.5, 'A');

Type:
(i32, f64, char)

-----------------------------------------------------

2) Accessing elements (Index-based):

tuple.0
tuple.1
tuple.2

-----------------------------------------------------

3) Destructuring:

let (a, b, c) = tuple;

-----------------------------------------------------

4) Mutable tuple:

let mut tuple = (10, 20);

tuple.0 = 100;

-----------------------------------------------------

5) Nested tuple:

let nested = ((1,2), (3,4));

-----------------------------------------------------

6) Returning multiple values from function:

fn example() -> (i32, i32)

-----------------------------------------------------

7) Tuple can contain array, vector, string etc.

-----------------------------------------------------

8) Unit tuple:

()
This is called unit type.
Equivalent to "void" in other languages.

*/

fn main() {

    // 1️ Initialization
    let tup = (10, 20.5, 'A');
    println!("Tuple: {:?}", tup);

    // 2️ Accessing elements
    println!("First: {}", tup.0);
    println!("Second: {}", tup.1);
    println!("Third: {}", tup.2);

    // 3️ Destructuring
    let (x, y, z) = tup;
    println!("Destructured: {x}, {y}, {z}");

    // 4️ Mutable tuple
    let mut tup2 = (5, 10);
    tup2.0 = 50;
    println!("Modified tuple: {:?}", tup2);

    // 5️ Nested tuple
    let nested = ((1, 2), (3, 4));
    println!("Nested first inner first value: {}", nested.0.0);

    // 6️ Function returning tuple
    let result = calculate();
    println!("Sum: {}, Product: {}", result.0, result.1);

    // Destructure return
    let (sum, product) = calculate();
    println!("Sum: {sum}, Product: {product}");

}

fn calculate() -> (i32, i32) {
    let a = 5;
    let b = 10;

    (a + b, a * b)
}
