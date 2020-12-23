use std::io; // We are using the io library from the standard library

fn main() {
    println!("Guess the number!"); // println! is a macro (you can tell by the !)

    println!("Please input your guess.");

    let mut guess = String::new(); // Variables are declared using 'let'.
                                   // Variables in Rust are immutable by default, but can
                                   // be declared as mutable with 'let mut'.

    // :: means that the function is an 'associated function', meaning that this function
    // is for all instances of the type, rather than an instance of that type.
    // 'new' is a function present on many types.

    io::stdin()
        .read_line(&mut guess) // read_line sets the string, but also returns an io::Result type
        .expect("Failed to read line"); // The Result type has an except method you can call.

    // & means that we are passing a reference to data, rather than a copy of the data. This is
    // similar to references in C++. Like variables, references are also immutable by default.
    println!("You guessed: {}", guess); // {} is similar to Python, in that you use them for string formatting.
}

