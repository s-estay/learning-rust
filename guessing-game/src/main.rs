use std::io; // `std`: standard library
             // `io`: input/output library
use rand::Rng; // add Rng trait

fn main() {
    println!("guess the number!"); // macro that prints a string to the screen
    let secret_number = rand::thread_rng().gen_range(1..=100); // `rand::thread_rng` call the random number generator
                                                               // method `gen_range` is defined by the Rng trait 
                                                               // `gen_range argument = start..=end
    println!("the secret number is: {secret_number}");
    println!("input your guess"); 
    let mut guess = String::new(); // use `let` statement to create a variable
                                   // `let apples = 5;`
                                   // in Rust, variables are immutable by default
                                   // to make a variable mutable, we add `mut` before the variable name
                                   // `String::new` is a function that returns a new instance of a string
                                   // the `::new` function creates a new empty string
    io::stdin() // call the `stdin` function from the `io` module
                // this will allow us to handle user input
                // without `use std::io` we can call stdin like this: `std::io::stdin()`
        .read_line(&mut guess) // call the `read_line` method on the standard input handler
                               // pass `&mut guess` as the argument to `read_line`
                               // The & indicates that this argument is a reference
                               // a reference a way to let multiple parts of your code access one piece of data,
                               // without needing to copy that data into memory multiple times
                               // references are immutable by default, hence `&mut guess` rather than `&guess`
        .expect("failed to read line"); // `read_line` returns a Result value
                                        // Result is an enumeration (enum)
                                        // an enumeration is a type that can be in one of multiple possible states
                                        // we call each possible state a variant
                                        // Result’s variants are Ok and Err
                                        // the Ok variant indicates the operation was successful,
                                        // and inside Ok is the successfully generated value
                                        // the Err variant means the operation failed,
                                        // and Err contains information about how or why the operation failed
                                        // `expect` is a method of Result
                                        // if Result is an Err value, `expect` will cause the program to crash
                                        // and display the message that you passed as an argument to expect
                                        // if Result is an Ok value,
                                        // `expect` will take the return value that Ok is holding and return that value
                                        // if you don’t call `expect`, the program will compile, but you’ll get a warning
                                        // the right way to suppress the warning is to actually write error-handling code
    println!("you guessed: {guess}"); // when printing the value of a variable, the variable name can go inside the curly brackets
                                      // `let x = 5;`
                                      // `let y = 10;`
                                      // `println!("x = {x} and y + 2 = {}", y + 2);`
                                      // this code would print: x = 5 and y + 2 = 12
}
