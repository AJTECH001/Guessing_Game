# Guess the Number Game

## Overview

This is a web-based number guessing game built with Rust and Leptos. The application generates a random number between 1 and 100, and the player must guess it through an interactive web interface. After each guess, the program provides real-time feedback on whether the guess was too high, too low, or correct. The game continues until the player guesses the correct number, with the option to play again.

## Features

- **Web-based interface**: Modern, interactive frontend built with Leptos
- **Real-time feedback**: Instant responses without page reloads
- **Reactive UI**: Dynamic updates using Leptos signals
- **Input validation**: Handles invalid inputs gracefully with user-friendly messages
- **Game state management**: Tracks game progress and enables restart functionality
- **Responsive design**: Clean, accessible interface for all devices
- **WebAssembly powered**: Runs Rust code directly in the browser

## Technology Stack

### Frontend Framework

- **Leptos** — Modern Rust web framework for building reactive web applications
- **WebAssembly** — Enables Rust code execution in the browser

### Core Dependencies

The program uses the following Rust crates:

- `leptos` — Reactive web framework with server-side rendering capabilities
- `rand` — For generating cryptographically secure random numbers
- `web-sys` — WebAssembly bindings for Web APIs
- `console_error_panic_hook` — Better error reporting in the browser console

Ensure these dependencies are included in your `Cargo.toml`:

```toml
[dependencies]
leptos = { version = "0.6", features = ["csr"] }
console_error_panic_hook = "0.1.7"
wasm-bindgen = "0.2"
leptos_meta = "0.6"
leptos_router = "0.6"
rand = "0.8.5"
colored = "2.0.0"
```

## How to Run

### Prerequisites

Ensure you have the following installed:

- **Rust** (latest stable version via [rustup](https://rustup.rs/))
- **Trunk** — WebAssembly web application bundler
  ```bash
  cargo install trunk
  ```
- **WebAssembly target**:
  ```bash
  rustup target add wasm32-unknown-unknown
  ```

### Setup Instructions

1. **Clone the Project**

   ```bash
   git clone https://github.com/AJTECH001/Guessing_Game.git
   cd Guessing_Game
   ```

2. **Install Dependencies**
   Dependencies will be automatically downloaded when you build the project.

3. **Create index.html**
   Create an `index.html` file in your project root:

   ```html
   <!DOCTYPE html>
   <html>
     <head>
       <meta charset="UTF-8" />
       <title>Guessing Game</title>
       <link rel="stylesheet" href="style.css" />
       <link data-trunk rel="rust" data-bin="guessing_game" />
     </head>
     <body></body>
   </html>
   ```

4. **Run the Development Server**

   ```bash
   trunk serve
   ```

5. **Open in Browser**
   Navigate to `http://localhost:8080` to play the game.

### Building for Production

```bash
trunk build --release
```

## How to Play

1. **Start the Game**: Open the web application in your browser
2. **Make Your Guess**: Enter a number between 1 and 100 in the input field
3. **Get Feedback**: The game will tell you if your guess is:
   - **"Too small!"** — Your guess is lower than the secret number
   - **"Too big!"** — Your guess is higher than the secret number
   - **"You win!"** — You've guessed correctly!
4. **Play Again**: Click the "Play Again" button to start a new game with a different secret number

## Code Architecture

### Component Structure

The application uses Leptos's component-based architecture:

**Main App Component (`App`)**:

- **State Management**: Uses Leptos signals for reactive state

  - `secret_number`: The randomly generated target number
  - `guess`: Current user input
  - `message`: Feedback message to display
  - `game_over`: Boolean flag for game completion state

- **Event Handlers**:

  - `handle_submit`: Processes user guesses and updates game state
  - `reset_game`: Resets all state for a new game

- **Reactive UI**: Automatically updates based on state changes

### Key Features Implementation

**Input Validation**: Parses user input and handles invalid entries gracefully

```rust
let guess_value = guess.get().trim().parse::<u32>();
match guess_value {
    Ok(num) => { /* Process valid guess */ }
    Err(_) => { /* Handle invalid input */ }
}
```

**Game State Management**: Uses pattern matching for clean game logic

```rust
match num.cmp(&secret_number.get()) {
    Ordering::Less => { /* Too small */ }
    Ordering::Greater => { /* Too big */ }
    Ordering::Equal => { /* Winner! */ }
}
```

**Reactive UI Updates**: Leverages Leptos signals for automatic re-rendering

```rust
<p class=move || if message.get().contains("win") { "win" } else { "error" }>
    {move || message.get()}
</p>
```

## Development Notes

- **Debug Mode**: The secret number is currently displayed for testing purposes. Remove the debug line in production:

  ```rust
  // Remove this line for production:
  <p>"The secret number is: " {move || secret_number.get()}</p>
  ```

- **Error Handling**: The application includes comprehensive error handling for user input and uses `console_error_panic_hook` for better debugging in the browser console.

- **Performance**: The game runs entirely in the browser using WebAssembly, providing near-native performance for the game logic.

## Example Gameplay

```
Guess the Number!
The secret number is: 42 (debug mode)

[Input field: "50"] [Submit Guess]
Too big!

[Input field: "30"] [Submit Guess]
Too small!

[Input field: "42"] [Submit Guess]
You win!

[Play Again] (button enabled)
```

## Future Enhancements

Potential improvements for the game:

- **Guess counter**: Track the number of attempts
- **Difficulty levels**: Different number ranges (1-10, 1-1000, etc.)
- **High scores**: Local storage for best attempt records
- **Animations**: CSS transitions for better user experience
- **Sound effects**: Audio feedback for guesses and wins
- **Themes**: Dark/light mode toggle

## Contributing

This project serves as an excellent introduction to:

- **Leptos framework fundamentals**
- **WebAssembly development with Rust**
- **Reactive programming patterns**
- **Modern web application architecture**

Feel free to contribute improvements, bug fixes, or new features!
