/*  HASHSET IN RUST

HashSet is a collection of unique values.

It stores only keys (no key-value pairs).

Internally implemented using HashMap.

For using HashSet:
use std::collections::HashSet;

-----------------------------------------------------

1) Initialization:

let set: HashSet<i32> = HashSet::new();

-----------------------------------------------------

2) Insert value:

set.insert(value);

If value already exists → nothing happens.

-----------------------------------------------------

3) Check if value exists:

set.contains(&value);

-----------------------------------------------------

4) Remove value:

set.remove(&value);

-----------------------------------------------------

5) Length:

set.len();

-----------------------------------------------------

6) Clear set:

set.clear();

-----------------------------------------------------

7) Iteration:

for value in &set {
    println!("{value}");
}

-----------------------------------------------------

8) HashSet does NOT allow duplicates.

-----------------------------------------------------

9) HashSet does NOT guarantee order.

-----------------------------------------------------

10) If order is required → use BTreeSet.

*/
use std::collections::HashSet;
use std::io;

fn main() {

    // 1️ Initialization
    let mut set: HashSet<i32> = HashSet::new();

    // 2️ Insert values
    set.insert(10);
    set.insert(20);
    set.insert(30);
    set.insert(10); // duplicate, ignored

    println!("Set: {:?}", set);

    // 3️ Check existence
    let value = 20;
    if set.contains(&value) {
        println!("20 exists");
    }

    // 4️ Remove value
    set.remove(&30);
    println!("After removal: {:?}", set);

    // 5️ Length
    println!("Length: {}", set.len());

    // 6️ Iteration
    for value in &set {
        println!("Value: {value}");
    }

    // 7️ User input example
    println!("Enter 3 numbers:");

    let mut user_set: HashSet<i32> = HashSet::new();

    for _ in 0..3 {
        let mut raw = String::new();
        io::stdin().read_line(&mut raw).expect("Invalid input");

        let value: i32 = raw.trim().parse().expect("Enter valid integer");

        user_set.insert(value);
    }

    println!("User set: {:?}", user_set);

}
