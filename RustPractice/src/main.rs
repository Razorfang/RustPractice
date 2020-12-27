use std::io; // We are using the io library from the standard library
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!"); // println! is a macro (you can tell by the !)

    println!("Please input your guess.");

    let mut guess = String::new(); // Variables are declared using 'let'.
                                   // Variables in Rust are immutable by default, but can
                                   // be declared as mutable with 'let mut'.

    let secret_number = rand::thread_rng().gen_range(1, 10); // We didn't use 'mut', so this data is immutable.

    // :: means that the function is an 'associated function', meaning that this function
    // is for all instances of the type, rather than an instance of that type.
    // 'new' is a function present on many types.

    io::stdin()
        .read_line(&mut guess) // read_line sets the string, but also returns an io::Result type
        .expect("Failed to read line"); // The Result type has an except method you can call.

    // & means that we are passing a reference to data, rather than a copy of the data. This is
    // similar to references in C++. Like variables, references are also immutable by default.
    println!("You guessed: {}", guess); // {} is similar to Python, in that you use them for string formatting.

    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    // This is called 'shadowing', where we we re-use variables. This is useful for converting
    // types.


    match guess.cmp(&secret_number)
    {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => { println!("You win!"); break;},
    }
    // Ordering is an enum, like Result. It can be Less, Greater, or Equal.
    // cmp compares two values, and can be called on anything that can be compared.
    // A match expression takes a returned enum and runs a lambda based on the value of the enum.
    // This is kind of like a switch statement, I guess.
}


