mod chapter_1;

use std::io;

fn main() -> Result<(), String> {
    choose_program()?;
    Ok(())
}

fn choose_program() -> Result<(), String> {
    println!(
        "Choose a program to run:

Chapter 1: A) Hello Window

Type in the chapter number, along with the program letter (e.g. 1A)."
    );
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .map_err(|e| e.to_string())?;

    // Removes endline
    input.pop();

    match input.as_str() {
        "1A" => chapter_1::hello_window::run()?,
        _ => println!("Invalid input {}.", input),
    }

    Ok(())
}
