# 🦀 Hello, Ferris! - A Simple Rust Console App

Welcome to **Hello, Ferris!**, a beginner-friendly Rust application that greets the user with a personalized message. This project is designed to demonstrate the fundamentals of Rust programming, including handling user input, string manipulation, and writing unit tests.

---

## 📋 Project Description

This project prompts the user to input their name, and then it returns a personalized greeting in the format:

```

Hello, Ferris! My name is {name}!

```

If the user doesn't provide a name, the program notifies them that no input was received.

---

## 🛠 Features

- Dynamic greeting based on user input.
- Handles empty input gracefully with appropriate messages.
- Includes unit tests to validate functionality.
- Simple and clean Rust code for beginners to understand.

---

## 🎯 Learning Objectives

Through this project I learned:

- How to read and handle user input using `std::io`.
- String formatting and manipulation in Rust.
- Writing unit tests to verify program functionality.

---

## 🧑‍💻 How to Run the Project

### Prerequisites
- Install [Rust](https://www.rust-lang.org/tools/install).

### Steps

1. Clone the repository:
```bash
git clone https://github.com/Logan-Toms/hello_ferris.git
cd hello_ferris
```

2. Run the application:
```bash
cargo run
```

Follow the on-screen instructions to enter your name and receive a personalized greeting.

---

## 🧪 Running Tests

This project includes unit tests for the `greet` function. To run the tests, use:

```bash
cargo test
```

Expected output for the tests:

- **Input**: `"Rustation"` -> **Output**: `"Hello, Ferris! My name is Rustation!"`
- **Input**: `""` -> **Output**: `"Hello, Ferris! My name is !"`

---

## 📂 Project Structure

```
hello_ferris/
├── src/
│   └── main.rs      # Contains the main application logic and tests
├── Cargo.toml       # Project dependencies and metadata
└── README.md        # Project documentation
```

---

## 🖼️ Example Output

### Input:

```
Hello! What's your name?
Logan
```

### Output:

```
Hello, Ferris! My name is Logan!
```

---

This project is part of my learning journey in Rust and was inspired by the Rust mascot, Ferris! 🦀
