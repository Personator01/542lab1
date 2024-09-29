//
// script_gen.rs
// Generation of the script from a configuration file.
//
// Author: Ty Butler
// Email: ty@wustl.edu
//
use std::fs::File;
use std::io::{BufReader, BufRead};

/// A PlayConfig consists of pairs containing the name of a character and the file containing their
/// lines.
type PlayConfig = Vec<(String, String)>;

/// Token indices of part name and filename within a config line
const PART_NAME_IDX: usize = 0;
const PART_FILE_IDX: usize = 1;

/// Line offsets into a configuration file for the specified values
const PLAY_TITLE_IDX: usize = 0;
const PLAY_PARTS_IDX: usize = 1;

/// Expected number of tokens on a line of a configuration file
const TOKENS_PER_LINE: usize = 2;

/// Adds a line to the script, checking if it has valid format
fn add_script_line(play: &mut Play, line: &String, speaker: &String) {
    if let Some((lineno, line_rest)) = line.split_once(char::is_whitespace) {
        let lineno = lineno.trim();
        match lineno.parse::<usize>() {
            Ok(lineno) => {
                let line_rest_trim = line_rest.trim();
                if !line_rest.is_empty() {
                    play.push((lineno, speaker.clone(), line_rest_trim.to_string()));
                }
            }, 
            Err(_) => {
                if DO_WARN.load(Ordering::SeqCst) {
                    println!("Could not parse token {} as integer (usize)", lineno);
                }
            }
        }
    } else {
        if DO_WARN.load(Ordering::SeqCst) {
            println!("Line is malformed: \"{}\" could not be split into line number and line", line);
        }
    }
}

/// Gets lines from the given file and stores them in the given vec
/// Will not store blank lines.
fn grab_trimmed_file_lines(filename: &String, lines: &mut Vec<String>) -> Result<(), u8> {
    let file = match File::open(filename) {
        Ok(f) => f,
        Err(m) => { 
            println!("Failed to open file \"{}\", {:?}", filename, m);
            return Err(ERR_SCRIPT);
        }
    };
    let mut line = String::new();
    let mut reader = BufReader::new(file);
    loop {
        match reader.read_line(&mut line) {
            Err(e) => { println!("Error reading from file {}, {:?}", filename, e); return Err(ERR_SCRIPT); },
            Ok(0) => return Ok(()),
            Ok(_) => {
                let line_trim = line.trim();
                if !line_trim.is_empty() {
                    lines.push(line_trim.to_string());
                }
            } 
        }
        line.clear();
    }
}

/// Processes a config file, adding the lines for the defined parts to the given play
fn process_config(play: &mut Play, config: &PlayConfig) -> Result<(), u8> {
    for (part_name, part_file) in config {
        let mut lines: Vec<String> = Vec::new();
        grab_trimmed_file_lines(part_file, &mut lines)?;
        lines.iter().for_each(|line| add_script_line(play, line, part_name)); // Adds all lines 
    }
    return Ok(());
}

/// Parses and adds a part configuration entry
fn add_config(conf_line: &String, config: &mut PlayConfig) {
    let tokens: Vec<&str> = conf_line.split_whitespace().collect();
    if tokens.len() != TOKENS_PER_LINE && DO_WARN.load(Ordering::SeqCst) {
        println!("Line \"{}\" does not contain two tokens", conf_line);
    }
    if tokens.len() >= 2 {
        config.push((String::from(tokens[PART_NAME_IDX]), String::from(tokens[PART_FILE_IDX])));
    }
}

/// Parses a configuration file into a PlayConfig
fn read_config(filename: &String, play_name: &mut String, config: &mut PlayConfig) -> Result<(), u8> {
    let mut lines: Vec<String> = Vec::new();
    grab_trimmed_file_lines(filename, &mut lines)?;
    if lines.len() < 2 {
        println!("Config file {} does not contain any parts", filename);
        return Err(ERR_SCRIPT);
    } else {
        *play_name = lines[PLAY_TITLE_IDX].clone();
        lines.iter().skip(PLAY_PARTS_IDX).for_each(|line| add_config(line, config));
        return Ok(());
    }
}

/// Generates a script from the given configuration file
fn script_gen(conf_file: &String, play: &mut Play, title: &mut String) -> Result<(), u8> {
    let mut config: PlayConfig = Vec::new();
    let mut play_name = String::new();
    read_config(conf_file, &mut play_name, &mut config)?;
    process_config(play, &config)?;
    *title = play_name.clone();
    return Ok(());
}
