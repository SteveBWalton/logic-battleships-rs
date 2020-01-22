// Attempt to convert the Python battleships solver into Rust.
//@file battleships.rs
//@author Steve Walton

// This allows camelCase variables and functions.
#![allow(non_snake_case)]


/// The main entry point for the "battleships" program.
fn main()
{
    // Fetch the command line arguments.
    let args: Vec<String> = std::env::args().collect();
    println!("{:?}", args);

    if args.len() == 1
    {
        // No arguments, execute a synchronise.
        println!("No arguments.");
    }
    else
    {
        println!("There are {} arguments", args.len() - 1);

        // Decode the command parameters (ignore the program name).
        // for (int parameter = 1; parameter < argc; parameter++)
        for parameter in 1..args.len()
        {
            println!("Parameter {} is {}", parameter, args[parameter]);

            let firstChar = &args[parameter][..1];
            if firstChar == "-"
            {
                let otherChar = &args[parameter][1..];
                println!("Switch is {}", otherChar);

                /*
                match otherChar
                {
                "d" =>
                {
                    // Delete the files that follow on the command line.  Don't pass to Python.
                    isDeleteMode = true;
                    isPassToPython = false;
                },

                "v" =>
                {
                    // Get the level from the next parameter, if exists.
                    let parameter = parameter + 1;
                    println!("The level is {}", args[parameter]);
                    /*
                    if (parameter < argc)
                    {
                        sscanf(argv[parameter], "%d", &verbose_);
                    }
                    else
                    {
                        verbose_ = REPORT_FOLDER;
                    }
                    */
                },

                _ =>
                {
                    // Unknown arguments, pass this to the python program.
                    isPassToPython = true;
                },
                }
                */
            }
        }
    }

    // Return success.
    println!("Goodbye.");
    return;
}
