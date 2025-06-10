use chrono::{Local, Utc};
use std::fs;
//Today we are learning new topic called Options and Result, As you see we are slowing getting into the rust

/*
* What are Options?
* It is a Enum which provide Some None. And it is provided by the rust lib
* It is Used for Null, Nill or None values from a function which help to handle the values..
*
* It's act like a option means a function can return some value or null, none values. So this None
* or nill values are handle by the "Option" enum
*
* Let's Understand by some example
*
* The Option Enum Looks like this, Under the Hood
*
* pub Option<T> {
*   Some(T),
*   None
* }
*
*/

fn main() {
    let result: Option<i8> = find_first_a(String::from("Rohit vardhan"));

    //This we have learned in Enums called "patten Matching", And so in this way we can handle Errors values
    match result {
        Option::None => println!("a not found"),
        Option::Some(value) => println!("a is found at index {}", value),
    };

    /*
     * No we are learning Result which is used to handle Errors, also know as Error Handling.
     * It's like a "try- catch method" in java, just like "Result" Enums are used for error handling.
     *
     * This will have two types, "OK" means the function worked right and "Err" means i has throw or return an error
     * Let's get understand by the example
     *
     */
    let context = fs::read_to_string("hello.txt");

    match context {
        Ok(value) => println!("{}", value),
        Err(error) => panic!("Content not found {}", error),
    };

    let localtime = Local::now();
    println!("{}", localtime);

    let utc_time = Utc::now();
    println!("{}", utc_time);
}

/*
* what's happening, so when ever a funtion can have a return value as "null" or "nill", insteded of
* retuning the "-1" number we are returning "None". Because if a is does not present in the 'str'
* we can't return "none" value, Rust don't have 'nul' or 'nill' values.
*
* So to handle this errors or none values we are using Options Enums
*/

//This is the perious example where we learn about loops
fn find_first_a(str: String) -> Option<i8> {
    for (inddex, char) in str.chars().enumerate() {
        if char == 'a' {
            //Here you can see we are return the i8  number but under "some" enus
            return Some(inddex as i8);
        }
    }
    //here you can see, in perious we have return an number '-1'. Which was not a best pratices
    None
}

/*
* No we are learning Result which is used to handle Errors, also know as Error Handling.
* It's like a "try- catch method" in java, just like "Result" Enums are used for error handling.
*
* This will have two types, "OK" means the function worked right and "Err" means i has throw or return an error
* Let's get understand by the example
*
*/
