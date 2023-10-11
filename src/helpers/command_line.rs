use std::io::{stdin, stdout};

use crossterm::{
    style::{Color, ResetColor, SetForegroundColor},
    ExecutableCommand,
};
// Get user request
pub fn get_user_response(question: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut stdout: std::io::Stdout = stdout();

    // Print question in specific color
    stdout.execute(SetForegroundColor(Color::Blue))?;
    println!("\n{}", question);

    // Reset color
    stdout.execute(ResetColor)?;

    // Read user input
    let mut user_response: String = String::new();
    stdin()
        .read_line(&mut user_response)
        .expect("Failed to read response");

    // Trim whitespace and return
    Ok(user_response.trim().to_string()) 
}
