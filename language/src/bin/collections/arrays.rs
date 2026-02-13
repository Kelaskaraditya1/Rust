/*  ARRAY IN RUST

Array is a fixed-size collection of elements of the same type.

Key Points:

1) Array size must be known at compile time.
2) All elements must be of the same type.
3) Stored on stack (usually).
4) Cannot grow or shrink.
5) Type syntax: [type; size]

-----------------------------------------------------

1) Initialization:

let arr = [1, 2, 3, 4, 5];
Type: [i32; 5]

Explicit type:
let arr: [i32; 3] = [10, 20, 30];

-----------------------------------------------------

2) Initialize with same value:

let arr = [0; 5];
// creates [0,0,0,0,0]

-----------------------------------------------------

3) Accessing elements:

arr[index]

Example:
arr[0]

-----------------------------------------------------

4) Safe access:

arr.get(index)
// returns Option<&T>

-----------------------------------------------------

5) Length:

arr.len()

-----------------------------------------------------

6) Iteration:

a) Immutable iteration:

for value in &arr {
    println!("{value}");
}

b) With index:

for (index, value) in arr.iter().enumerate() {
    println!("{index} => {value}");
}

c) Mutable iteration:

for value in &mut arr {
    *value += 10;
}

-----------------------------------------------------

7) Array to Slice:

&arr[..]
&arr[1..3]

-----------------------------------------------------

8) Arrays implement Copy if elements implement Copy.

let a = [1,2,3];
let b = a; // copied

-----------------------------------------------------

9) When to use array:

- Fixed size data
- Small datasets
- Lookup tables
- Known compile-time size

If dynamic size is needed → use Vec

*/

use std::io;

fn main() {

    // 1 Initialization
    let arr = [10, 20, 30, 40, 50];
    println!("Array: {:?}", arr);

    // 2️ Explicit type
    let arr2: [i32; 3] = [1, 2, 3];
    println!("Array2: {:?}", arr2);

    // 3️ Same value initialization
    let arr3 = [0; 5];
    println!("Array3: {:?}", arr3);

    // 4️ Accessing elements
    println!("First element: {}", arr[0]);

    // 5️ Safe access
    match arr.get(10) {
        Some(value) => println!("Value: {value}"),
        None => println!("Index out of bounds"),
    }

    // 6️ Iteration (immutable)
    for value in arr {
        println!("Value: {value}");
    }

    // 7️ Iteration with index
    for (index, value) in arr.iter().enumerate() {
        println!("Index: {index}, Value: {value}");
    }

    // 8️ Mutable iteration
    let mut arr4 = [1, 2, 3];
    for value in &mut arr4 {
        *value += 10;
    }
    println!("Modified arr4: {:?}", arr4);

    // 9️ Slicing
    let slice = &arr[1..4];
    println!("Slice: {:?}", slice);

    //  User input example (Fixed size input)

    println!("Enter 3 numbers:");

    let mut input_arr = [0; 3];

    for i in 0..3 {
        let mut raw = String::new();
        io::stdin().read_line(&mut raw).expect("Invalid input");

        input_arr[i] = raw.trim().parse().expect("Enter valid integer");
    }

    println!("User array: {:?}", input_arr);

}
