//
// declarations.rs
// Contains useful type, constant, and variable definitions for use throughout the program
//
// Author: Ty Butler
// Email: ty@wustl.edu
//
use std::sync::atomic::AtomicBool;

/// A play consists of a vector of tuples containing the line number, name of the speaker, and line
/// itself.
type Play = Vec<(usize, String, String)>;

/// Minimum and maximum number of command line arguments
const MIN_ARGS: usize = 2;
const MAX_ARGS: usize = 3;

/// Indices of individual command line arguments
const PROGRAM_NAME_IDX: usize = 0;
const CONFIG_FILE_IDX: usize = 1;
const DO_WARN_IDX: usize = 2;

/// Program error codes
const ERR_BAD_CMD: u8 = 1;
const ERR_SCRIPT: u8 = 2;

/// Whether or not to print verbose output
static DO_WARN: AtomicBool = AtomicBool::new(false);
