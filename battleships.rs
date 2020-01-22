// Attempt to convert the Python battleships solver into Rust.
//@file battleships.rs
//@author Steve Walton

// This allows camelCase variables and functions.
#![allow(non_snake_case)]

mod battleship;

enum CommandLine
{
    Searching,
    GameIndex
}

/// The main entry point for the "battleships" program.
fn main()
{
    // Fetch the command line arguments.
    let args: Vec<String> = std::env::args().collect();
    println!("{:?}", args);

    let mut game = battleship::Game::new();

    // Decode the command parameters (ignore the program name).
    // for (int parameter = 1; parameter < argc; parameter++)
    let mut status = CommandLine::Searching;
    for parameter in 1..args.len()
    {
        match status
        {
            CommandLine::Searching =>
            {
                println!("Parameter {} is {}", parameter, args[parameter]);

                let firstChar = &args[parameter][..1];
                if firstChar == "-"
                {
                    let otherChar = &args[parameter][1..];
                    // println!("Switch is {}", otherChar);

                    match otherChar
                    {
                        "g" =>
                        {
                            status = CommandLine::GameIndex;
                        },

                        _ =>
                        {
                            // Unknown arguments.

                        },
                    }
                }
            }

            CommandLine::GameIndex =>
            {
                game.index = args[parameter].parse::<i32>().unwrap();
                status = CommandLine::Searching;
            }
        }
    }

    println!("game index is {}", game.index);

    // Return success.
    println!("Goodbye.");
    return;
}
