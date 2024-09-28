## CSE 542 Lab 1
Ty Butler
ty@wustl.edu

# Design and Implementation
The program was designed according to the assignment page. It is divided among three files, `declarations.rs` contains type and constant value declarations for use within the other files, as well as the static boolean which stores whether or not to print warnings. `main.rs` contains the program's entry point and handles the general control flow of the program. It parses and validates command line arguments, then makes calls to functions in `script_gen.rs` to process the given config file. Afterwards, it prints out the resultant play. `script_gen.rs` contains functions for processing a configuration file, and generating a play script from the relevant part files. It loads a config file, parses it line by line into the type `PlayConfig`, and from there uses the config file to open the appropriate part files and build the full play script.

The main aspect of this assignment that stuck out was the use of the `include!` macro rather than having the program separated into modules. Is there a specific reason for the instructions to use `include!`? Or is is simply that we have not yet covered that language feature in the course material? Also, why is our error type a `u8` rather than a string or a polymorphic error type.

I ran into a few bugs after writing the majority of the code. One was that one of my functions (script_gen) took a reference to a string which it treated as the title of the play, but when it was called it was passed a reference to the config file instead, leading to the program not finding the specified config file. This was somewhat a consequence of the program using mutable references as in-parameters, with the variable being declared before a function call, and then being set by passing a mutable reference into a function call. Thus even if the variable was never set by the function call, it would still be able to be used later in the code, causing errors when a later function is given an empty string, for example. It is for this reason that I prefer to use regular return values rather than in-parameters, as the compiler checks that the variable is actually initialized.

Another bug I ran into was that `grab_trimmed_file_lines` would, instead of returning the lines in a file sequentially like this: [1, 2, 3, 4, 5], would return each line along with the ones that came before it, like this: [1, 1 2, 1 2 3, 1 2 3 4, 1 2 3 4 5]. This was because I did not clear the buffer that held each line in between reading lines.

# Running The Program
## Send to linux lab
    rsync lab1.tar.gz username@shell.cec.wustl.edu:lab1.tar.gz
    ssh username@shell.cec.wustl.edu
    qlogin

## Extracting and running the project:
    tar -xzf lab1.tar.gz
    cd 542lab1
    cargo build
    cd text/hamlet_correct
    cargo run -- hamlet_ii_2_config.txt


# Testing The program

The `text` directory contains example text, the given hamlet excerpts, which have various changes made to test the program.
`hamlet_correct` contains the sample excerpts and config file.
`hamlet_config` contains various unusual config files
`hamlet_lines` contains part files with unusual formats.

## Testing the Command Line Interface

    cd text/hamlet_correct

Tests behavior when run without any config filename given

    cargo run 
Expected: usage message

Tests behavior when given normal config file

    cargo run -- hamlet_ii_2_config.txt
Expected: Play script

Tests behavior when given with an option that is not 'whinge'

    cargo run -- hamlet_ii_2_config.txt aaaa
Expected: usage message

Tests behavior when given a normal config file that does not produce any warnings 

    cargo run -- hamlet_ii_2_config.txt whinge
Expected: Play script

Tests behavior when given a nonexistent config file

    cargo run -- hhh.txt whinge
Expected: File not found message, error

## Testing the Configuration file

    cd text/hamlet_config

Tests program's ability to handle blank lines in a config file.

    cargo run -- hamlet_spaces.txt whinge
Expected: Play script

Tests program's behavior when given config file where a part does not have a part file specified

    cargo run -- hamlet_missing_file.txt whinge
Expected: Warning about line King not having two tokens, Play script without king

Tests program's behavior when given extra input on a part line
Ex, `King King.txt King2.txt` instead of `King King.txt`

    cargo run -- hamlet_extra_file.txt 
Expected: Play script 

Tests program's ability to report when it is given extra input on a part line

    cargo run -- hamlet_extra_file.txt whinge
Expected: Warning about line King not having two tokens, Play script 


Tests program's behavior when config file references nonexistent file 

    cargo run -- hamlet_incorrect_file.txt
Expected: File not found message, error

Tests program's behavior when encountering empty config file

    cargo run -- hamlet_blank.txt
Expected: Config file does not contain parts message, error

## Testing the part files
    cd text/hamlet_lines

Tests program's ability to handle blank lines in part file

    cargo run -- hamlet_spaces.txt whinge
Expected: Play script

Tests program's ability to handle out of order lines in part file

    cargo run -- hamlet_outoforder.txt
Expected: Play script

Tests program's ability to handle lines without numbers

    cargo run -- hamlet_nonumbers.txt
Expected: Play script, queen has no lines

Tests program's ability to report lines having no numbers

    cargo run -- hamlet_nonumbers.txt whinge
Expected: Many messages about not being able to parse words as integers, Play script, queen has no lines

Tests program's ability to handle parts with malformed line numbers
    
    cargo run -- hamlet_brokennumbers.txt whinge
Expected: Single message about not being able to parse value "1aaa" as an int, Play script, first line missing

Tests program's ability to handle part files which consist of only line numbers

    cargo run -- hamlet_nolines.txt
Expected: Play script, Rosencrantz has no lines

Tests program's ability to report lines consisting of only line numbers

    cargo run -- hamlet_nolines.txt whinge
Expected: Messages about lines being malformed, Play script, Rosencrantz has no lines
