// src/main.rs

// Function to perform fizz buzz and count occurrences
fn fizz_buzz_counts() -> (usize, usize, usize) {
    let mut fizz_count = 0;
    let mut buzz_count = 0;
    let mut fizz_buzz_count = 0;

    for i in 1..=301 {
        if i % 3 == 0 && i % 5 == 0 {
            println!("fizz buzz");
            fizz_buzz_count += 1;
        } else if i % 3 == 0 {
            println!("fizz");
            fizz_count += 1;
        } else if i % 5 == 0 {
            println!("buzz");
            buzz_count += 1;
        }
    }

    (fizz_count, buzz_count, fizz_buzz_count)
}

// Main function
fn main() {
    println!("Welcome to the bootcamp!");

    // Call the fizz buzz function and get counts
    let (fizz_count, buzz_count, fizz_buzz_count) = fizz_buzz_counts();

    // Print the counts
    println!("Number of times 'fizz' occurred: {}", fizz_count);
    println!("Number of times 'buzz' occurred: {}", buzz_count);
    println!("Number of times 'fizz buzz' occurred: {}", fizz_buzz_count);
}


