use std::sync::atomic::AtomicBool;

type Play = Vec<(usize, String, String)>;

const MIN_ARGS: usize = 2;
const MAX_ARGS: usize = 3;

const PROGRAM_NAME_IDX: usize = 0;
const CONFIG_FILE_IDX: usize = 1;
const DO_WARN_IDX: usize = 2;

const ERR_BAD_CMD: u8 = 1;
const ERR_SCRIPT: u8 = 2;

static DO_WARN: AtomicBool = AtomicBool::new(false);
