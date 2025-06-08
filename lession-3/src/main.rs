//Day-3 of learning the Structs and Enums in rust

//What is a Structs, In a simple ways struct which help to struct the data aslo we can think in this way that we can create a Data structures

/*
* Here is the example we have a User which as firstname, lastname and age
*
* Normally we wil create three variables like this :
*
*   let user_name:String = String::from("Rohit");
*   let user_lastname:String = String::from("Vardhan")
*   let user_age:i8 = 20;
*
* Here you can see, the above will take some many lines when we create n number of Users, but we can create a Struct so that all the user info is structured to geather, here is the implementation
* this is the Structure of the Struct
*
*   struct struct_name {
*       variables : Datatype
*   }
*
*   <--The struct_name should be begin with capital letter-->
*
* In Rust the struct is act like a class in java or javascript where we can implement static
* and non static methods etc.
*/

struct User {
    name: String,
    last_name: String,
    age: i8,
}

//Here is how you can create methods for struct

impl User {
    //What is "self", &self -> act as a "this" keyword in the java or javascript  which refer to the current object variables
    fn print_full_name(&self) {
        println!("{} {}", self.name, self.last_name);
    }

    // Here you can see we didn't add the "&self" parameter to the function, so it act like a static method
    fn debug() {
        println!("Degub");
    }

    //And we can also pass parameter other that the struct variables
    fn other_function(num: i8) -> i8 {
        num + num
    }
}

//What are Enums ?, The techinical defination is "It's Enumurates over various types of a Value". :(
/*
*  Lets us understand with simple exapmle :
*      As we have a User data we need to know the Gender of the user
*      Either it can be "Male or Female", can it have a Animale !
*      No right, So the ""Enums"" Store the two types Male and Female like this :
*
*   Un like other emuns we can store the value into the Enus in Rust
*
*/

enum Gender {
    Male(i8),
    Female(i8),
}

fn main() {
    //In this way you can create n users than create every variable single by single
    let user1 = User {
        name: String::from("Rohit"),
        last_name: String::from("Vardhan"),
        age: 21,
    };

    //By this we can access the struct bu the variable "user1"
    println!(
        "First name : {}, Last name {}, Age : {}",
        user1.name, user1.last_name, user1.age
    );

    //Here we can use the method of "user" struct
    user1.print_full_name();

    //Here is the example of Static function in struct
    User::debug();

    let number: i8 = User::other_function(3);
    println!("The rust of parameter functino with struct {}", number);
    /*
     * Here is the catch you can't print the whole user like this println!("{}", user1);
     * Because the "println!("")" don't know how to print a struct, but as we move on to the macros,
     * we will get to know how to print structs
     *
     * "What are macros !", I Don't know we will cover on further lessions
     *
     */

    //This is how to initilize the Enums
    let gender = Gender::Female(1);

    print_gender(gender);
}

/*
 * And print the value of the Enums, this is know as pattern matching syntax where it will match the enus and print or return values which we want to retur, here we are returning binary value like 1 or 0
 *
 * But this is not the best pratics of print or returning the values form enums
 *
 * And also  we can use this in struct
 *
 * struct struct_name {
 *   variable : Enum_name::type(value | not)
 * }
 *
 */

//Remeber for the best results and clean code use this function to print or return any enum value, Mainly if your printing
fn print_gender(gender: Gender) -> i8 {
    match gender {
        Gender::Male(value) => value,
        Gender::Female(value) => value,
    }
}

//That all for today see on the next lession
