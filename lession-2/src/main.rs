//Day-2 of learning some more variables, loops, conditionals and functions.Here you need to understand that rust is an Type-safe language, it means you need to have type to the variables,but rust can automatically detects the type and add at the compliation.
//
//For the best Pratices please mention Datatype to the variables, not only variables,use it in entire code.

// To run a file use "cargo run"

//Instided of learning syntax it better to learn by doing some probles.
fn main() {
    let name: String = String::from("Rohit Vardhan");
    // This function give the return value type of usize
    let length: usize = length_of_the_word(name);
    println!(
        "the Name is Rohit Vardhan and the no of characters {}",
        length
    );

    let fib_number: i8 = fib(4);
    println!("the Fibonacci number of 4, is {}", fib_number);

    let new_name: String = String::from("rohitvardhan");
    let index_of_first_a: i8 = give_first_a_in_word(new_name);
    println!(
        "The first index of a is {}, in rohit vardhan",
        index_of_first_a
    );
}

/*
* this is the structuer of the function
*
* fn function_name(variable : Datatype) -> return_type {
*   // Code to done a task
* }
*/

//Here we are creater a function to print the length of the characters
fn length_of_the_word(s: String) -> usize {
    //Here you can see we are not writing the return statemnt, because rust can implicitly add return statemnt if we dont mention the "return" and "colin ;l"
    s.chars().count()
}

/*
*
* this is how we can write if statement
*
* if condition {
*   //code...
* }
*
* ------if-else statement
*
* if condition {
*   //code...
* } else {
*   //code...
* }
*/

//Here is an another code to give the fibonacci sequency number by gving the index
fn fib(num: i8) -> i8 {
    if num <= 1 {
        return num;
    }
    fib(num - 1) + fib(num - 2)
}

/*
*
* here how to write loops
*
* for variable in start..end {
*   //Do some code
* }
*
* this is the standard version of writing for loop
*
*/

fn give_first_a_in_word(s: String) -> i8 {
    //Here is the another version of wrtitng for loop also refered as for-each loop in python, or java
    for (index, character) in s.chars().enumerate() {
        if character == 'a' {
            return index as i8;
        }
    }
    -1
}

//I this this much of problems are easy and we know the logic, so writing with the rust syntax make easy to remember. Tye some easy problems using the functions, loops, variavles and conditionals.
//I hope you like my explaination and sorry for the splling mistakes :(
