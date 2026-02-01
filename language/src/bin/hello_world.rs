/*                                                                                           RUST

1) Rust is a compiled programming language

2) for running a rust programme , first we have to compile the porgramme using "rustc <file name>"
which generates a .exe file than we have to execute the .exe file using "./<file name>.exe"

3) Rust is a Ahead of time compiler and the .exe is platform indepenent similar to java.
we have to compile just once and than the .exe file can be executed on any platform.

*/

/*
Like other programming Languages execution of rust as well begins from main function , println!() is macro and not a function call.
Macro is a peice of code written in the library and has a name , and whereever it has to be used , the name is being replaced by the set of lines defined in the macro definition.
 */

/*                                                                                          CARGO
1) cargo is a package manager , for building project and installing packages amd dependencies.

Cargo commands:

1) for creating a new rust product: "cargo new <project name>"
the name should be in small letter and '_' seprated , all the code remains in the /src all the compiled code remains in the /target/debug/<filename>.exe

2) go to project dir , for compiling the project: "cargo build", it builds the /target/debug folder which contains the .exe file
3) for running the file , go to /target/debug and than run ./<file name>.exe for running the file.

for simplicity , do "cargo build" -> compiling 
                 do "cargo run" -> for running the file in the /target/debug 

if made any changes in any file than can directly run the "cargo run" , it auto compiles the project and build the .exe file.

for building the release version or the build artifact use: "cargo build --release".

*/

fn main() {
    println!("Hello, Rust!");
}