use std::fs::File;
use std::path::PathBuf;
use std::collections::VecDeque;
use std::io::{self, Read, Write};

/// Possible zero-width characters
enum InstructionChar {
    MOVE_RIGHT = 0x180E,
    MOVE_LEFT = 0x200B,
    INCREMENT = 0x200C,
    INPUT = 0x200D,
    LOOP = 0xFEFF,
}

/// Interprets a string slice. Useful for loops.
fn run_str(raw_contents: &str, plane: &mut VecDeque<u8>, location: &mut usize, in_loop: bool) {
    let mut wait_for_loop_end = false;
    let mut input_last = false;
    'run: for (i, ch) in raw_contents.chars().enumerate() {
        // If loop condition is not satisfied, wait_for_loop_end is set to true. As long as
        // the current character is not the loop instruction, indicating ending the loop, it
        // skips that character. If wait_for_loop_end is set to true and the character is the
        // loop instruction, wait_for_loop_end is set to false and the character is skipped. 
        if wait_for_loop_end && !(ch as u32 == 0xFEFF) {
            continue 'run;
        } else if wait_for_loop_end {
            wait_for_loop_end = false;
            continue 'run;
        }
        // In order to correctly interpret inputs vs outputs, whenever the input instruction
        // is seen, it sets input_last to true. It is only on the next turn that the
        // interpreter sees whether the next character is the input instruction or not, and
        // the interpeter can decide whether to run INPUT or OUTPUT.
        if ch as u32 == InstructionChar::INPUT {
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
            InstructionChar::MOVE_LEFT => {
                if *location == 0 {
                    plane.push_front(0);
                } else {
                    *location -= 1;
                }
            },
            InstructionChar::MOVE_RIGHT => {
                if *location == plane.len() - 1 {
                    plane.push_back(0);
                    *location += 1;
                } else {
                    *location += 1;
                }
            },
            InstructionChar::INCREMENT => {
                if plane[*location] == u8::MAX {
                    plane[*location] = 0;
                } else {
                    plane[*location] += 1;
                }
            },
            InstructionChar::LOOP => {
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

pub fn main() -> io::Result<()> {
    let path = PathBuf::from(
        std::env::args_os().nth(1).expect("No file argument passed in")
    );
    let mut file = File::open(&path)?;
    let mut raw_contents = String::new();
    file.read_to_string(&mut raw_contents)?;
    let mut plane = VecDeque::from([0u8]);
    let mut location: usize = 0;
    run_str(raw_contents.as_str(), &mut plane, &mut location, false);
    Ok(())
}
