use std::io::prelude::*;
use std::process::{Command, Stdio};

static PANGRAM: &'static str = 
"the quick brown fox jumps over lazy dog\n";

fn main() {
    // Spawn the `wc` command
    let process = match Command::new("wc")
                                .stdin(Stdio::piped())
                                .stdout(Stdio::piped())
                                .spawn() {
        Err(why) => panic!("couldn't spawn wc: {}", why),
        Ok(process) => process,
    };


    // Write a string to the `stdin` of `wc`

    // `stdin` has type `Option<ChildStdin>`, but since we know this instance must have
    // one, we can directly `unwrap` it.

    match process.stdin.unwrap().write_all(PANGRAM.as_bytes()) {
        Err(why) => panic!("couldn't write to wc stdin: {}", why),
        Ok(_) => println!("set panagram to wc"),
    }

    let mut s = String::new();
    match process.stdout.unwrap().read_to_string(&mut s) {
        Err(why) => panic!("couldn't read wc stdout: {}", why),
        Ok(_) => print!("wc responded with:\n{}", s),
    }
}