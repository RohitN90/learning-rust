//Day-1 of Learning Rust from Beginner to Advance and today i have learned about variables and different datatypes

fn main() {
    //This is how the variables are initilized in the Rust
    //By default all the variables in the Rust are constant variables
    //And here you can see "i8" is an "signed Integer" and there is also an "u8" is a Unsigned Integer
    let age: i8 = 20;
    let mut mutable_age: u8 = 20;

    //String is denoted as this notation
    let name: String = String::from("Hi, welcome to the Rust");

    //To print any variable this is the code
    println!("{}", name);
    println!("{}, {}", age, mutable_age);

    //We can not write the Camel case in the rust
}
