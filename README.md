## CSE 542 Lab 1
Ty Butler
ty@wustl.edu

# Design and Implementation
The program was designed according to the assignment page. It is divided among three files, `declarations.rs` contains type and constant value declarations for use within the other files, as well as the static boolean which stores whether or not to print warnings. `main.rs` contains the program's entry point and handles the general control flow of the program. It parses and validates command line arguments, then makes calls to functions in `script_gen.rs` to process the given config file. Afterwards, it prints out the resultant play. `script_gen.rs` contains functions for processing a configuration file, and generating a play script from the relevant part files. It loads a config file, parses it line by line into the type `PlayConfig`, and from there uses the config file to open the appropriate part files and build the full play script.

The main aspect of this assignment that stuck out was the use of the `include!` macro rather than having the program separated into modules. Is there a specific reason for the instructions to use `include!`? Or is is simply that we have not yet covered that language feature in the course material?

# Running The Program
## Extracting and running the project:
    tar -xzf lab1.tar.gz
    cd 542lab1
    cargo build
