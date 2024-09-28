//!
//! main.rs
//! Contains the entry point for the program and initial control flow.
//!
//! Author: Ty Butler
//! Email: ty@wustl.edu
//!
use std::env;
use std::sync::atomic::Ordering;

include!("declarations.rs");
include!("script_gen.rs");

/// Entry point of the program.
fn main() -> Result<(), u8>{
    let mut conf_file = String::new();
    parse_args(&mut conf_file)?;

    let mut title = String::new();
    let mut play: Play = Vec::new();

    script_gen(&conf_file, &mut play, &mut title)?;

    play.sort();
    recite(&title, &play);

    return Ok(());
}

/// Displays the proper command line usage of the program
fn usage(program_name: &String) {
    println!("Usage: {} <configuration_file_name> [whinge]", program_name);
}

/// Parses and validates command line arguments.
fn parse_args(config_name: &mut String) -> Result<(), u8> {
    let mut arguments: Vec<String> = Vec::new();
    for arg in env::args() {
        arguments.push(arg);
    }
    if arguments.len() < MIN_ARGS || arguments.len() > MAX_ARGS {
        usage(&arguments[PROGRAM_NAME_IDX]); // will panic if arguments does not contain any values. This is
        // correct because arguments should always contain the name of the program at index 0
        return Err(ERR_BAD_CMD);
    } else { 
        if arguments.len() == MAX_ARGS {
            if arguments[DO_WARN_IDX] != "whinge" {
                usage(&arguments[PROGRAM_NAME_IDX]); 
                return Err(ERR_BAD_CMD);
            }
            DO_WARN.store(true, Ordering::SeqCst);
        }
        *config_name = arguments[CONFIG_FILE_IDX].clone();
    }
    return Ok(());
}

/// Recites the play's lines
fn recite(title: &String, play: &Play) {
    let mut last_speaker = "";
    println!("{}", title);
    for (_lineno, speaker_name, line) in play {
        if speaker_name != last_speaker {
            last_speaker = speaker_name;
            println!("\n{}.", speaker_name);
        }
        println!("{}", line);
    }
}

