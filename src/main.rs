mod chapter_1;
mod shader;
use std::io;

fn main() -> Result<(), String> {
    choose_program()?;
    Ok(())
}

fn choose_program() -> Result<(), String> {
    // Clears terminal
    print!("{}[2J", 27 as char);

    println!(
        "Choose a program to run:

Chapter 1: A) Hello Window
           B) Hello Triangle
           C) Shaders
           D) Textures

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
        "1B" => chapter_1::hello_triangle::run()?,
        "1C" => chapter_1::shaders::run()?,
        "1D" => chapter_1::textures::run()?,
        _ => println!("Invalid input {}.", input),
    }

    Ok(())
}
