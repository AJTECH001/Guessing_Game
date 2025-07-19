# Guess the Number Game

## Overview
This is a simple command-line number guessing game written in Rust. The program generates a random number between 1 and 100, and the player must guess it. After each guess, the program provides feedback on whether the guess was too high, too low, or correct. The game continues until the player guesses the correct number.

## Features
- Generates a random number between 1 and 100.
- Accepts user input for guesses.
- Provides colored feedback (red for incorrect guesses, green for a win).
- Handles invalid inputs gracefully by prompting the user to try again.
- Continues until the correct number is guessed.

## Dependencies

The program uses the following Rust crates:
- `std::io` — For handling user input from the console.
- `rand` — For generating random numbers.
- `colored` — For adding colored text output to the console.

Ensure these dependencies are included in your `Cargo.toml`:

```toml  
[dependencies]  
rand = "0.8.5"  
colored = "2.1.0"  
```

# How to Run

## Prerequisites
Ensure you have Rust installed on your system. You can install it via rustup.

## Clone the Project
Clone or download the project to your local machine.

## Add Dependencies
Ensure the `rand` and `colored` crates are included in your `Cargo.toml` as shown above.

## Run the Program
1. Navigate to the project directory in your terminal.
2. Run the following command:
```bash  
cargo run  
```

# Play the Game

The program will display the secret number (for debugging purposes; you can comment out the line `println!("The secret number is: {}", secret_number);` to hide it).

## Instructions:
- Enter a number between 1 and 100 as your guess.
- The program will tell you if your guess is too high, too low, or correct.
- Continue guessing until you win.

## Code Structure

### Imports:
- `std::io`: Used for reading user input.
- `rand::Rng`: Provides random number generation functionality.
- `std::cmp::Ordering`: Used for comparing the user's guess with the secret number.
- `colored`: Adds color to the console output for better user experience.

### Main Function:
- Generates a random number (`secret_number`) between 1 and 100 using `rand::thread_rng().gen_range(1..=100)`.
- Enters a loop to:
  - Prompt the user for a guess.
  - Read and parse the input into a `u32` integer.
  - Compare the guess to the secret number using `guess.cmp(&secret_number)`.
  - Provide feedback using colored output:
    - **Red** for "Too small!" or "Too big!"
    - **Green** for "You win!"
- Exits the loop when the correct number is guessed.
- Handles invalid inputs (e.g., non-numeric input) by continuing the loop.

## Example Output

# Guess the number!

The secret number is: 42

Please input your guess.  
50  
**Too big!**

Please input your guess.  
30  
**Too small!**

Please input your guess.  
42  
**You win!**

# Notes

- The line `println!("The secret number is: {}", secret_number);` is included for debugging. Comment it out to hide the secret number during gameplay.
- Invalid inputs (e.g., letters or symbols) are handled by skipping to the next iteration of the loop, ensuring the program doesn't crash.
- The game uses colored output for a better user experience, with **red** for incorrect guesses and **green** for a win.