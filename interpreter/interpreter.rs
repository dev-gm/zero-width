use std::fs::File;
use std::path::PathBuf;
use std::collections::VecDeque;
use std::io::{self, Read, Write};

fn run_str(raw_contents: &str, plane: &mut VecDeque<u8>, location: &mut usize, in_loop: bool) {
    let mut wait_for_loop_end = false;
    let mut input_last = false;
    'run: for (i, ch) in raw_contents.chars().enumerate() {
        if wait_for_loop_end && !(ch as u32 == 0xFEFF) {
            continue 'run;
        } else if wait_for_loop_end {
            wait_for_loop_end = false;
            continue 'run;
        }
        if ch as u32 == 0x200D {
            if input_last {
                println!("OUT: {}", plane[*location]);
                input_last = false;
            } else {
                input_last = true;
            }
        } else if input_last {
            print!("IN: ");
            let _ = io::stdout().flush();
            let mut line = String::new();
            io::stdin().read_line(&mut line).expect("Failed to read input from STDIN");
            plane[*location] += line[..line.len()-1].parse::<u8>().expect("Failed to parse input from STDIN into integer");
            input_last = false;
        }
        match ch as u32 {
            0x180E => {
                if *location == 0 {
                    plane.push_front(0);
                } else {
                    *location -= 1;
                }
            },
            0x200B => {
                if *location == plane.len() - 1 {
                    plane.push_back(0);
                    *location += 1;
                } else {
                    *location += 1;
                }
            },
            0x200C => {
                if plane[*location] == u8::MAX {
                    plane[*location] = 0;
                } else {
                    plane[*location] += 1;
                }
            },
            0xFEFF => {
                if in_loop {
                    break 'run;
                }
                let mut loop_contents = "";
                if plane[*location] != 0 {
                    loop_contents = &raw_contents[i+18..];
                }
                while plane[*location] != 0 {
                    run_str(loop_contents, plane, location, true);
                }
                wait_for_loop_end = true;
            },
            _ => {},
        }
    }
}

pub fn run(raw_contents: String) {
    let mut plane = VecDeque::from([0u8]);
    let mut location: usize = 0;
    run_str(raw_contents.as_str(), &mut plane, &mut location, false);
}

pub fn main() -> io::Result<()> {
    let path = PathBuf::from(
        std::env::args_os().nth(1).expect("No file argument passed in")
    );
    let mut file = File::open(&path)?;
    let mut raw_contents = String::new();
    file.read_to_string(&mut raw_contents)?;
    run(raw_contents);
    Ok(())
}
