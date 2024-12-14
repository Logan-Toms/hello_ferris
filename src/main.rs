use std::io; // Brings the I/O module into scope

fn greet(name: &str) -> String {
    format!("Hello, Ferris! My name is {}!", name)
}

fn main() {
    println!("Hello! What's your name?"); // Prompts user for their name

    let mut name: String = String::new(); // Creates an empty string to hold users name
    if io::stdin().read_line(&mut name).is_ok() { // Reads input from console
        if name.trim().is_empty() { // If else statement to check for an empty string
            println!("Your didn't enter a name!");
        } else {
            println!("{}", greet(name.trim()));
        } 
    } else {
            println!("Somthing went wrong. Please try again.")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet_with_name() {
        let result: String = greet("Rustation");
        assert_eq!(result, "Hello, Ferris! My name is Rustation!");
    }

    #[test]
    fn test_greet_with_empty_name() {
        let result: String = greet("");
        assert_eq!(result, "Hello, Ferris! My name is !")
    }
}