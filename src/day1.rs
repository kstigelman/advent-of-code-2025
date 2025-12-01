use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn password (file_path: &str) -> u32 {

    let mut current_position = 50;
    let mut count = 0;

    let file = File::open (file_path);

    match file {
        Ok(f) => {
            let reader = BufReader::new (f);
            for line in reader.lines () {
                let current_line = match line {
                    Ok(s) => s,
                    Err (_) => String::new()
                };
                
                let mut move_instruction: Vec<&str> = Vec::new();

                let mut direction = 0;

                if current_line.find('L') != None {
                    direction = -1;
                    move_instruction = current_line.split('L').collect();
                }
                else if current_line.find('R') != None {
                    direction = 1;
                    move_instruction = current_line.split('R').collect();
                }

                assert_eq!(move_instruction.len(), 2);
                let trimmed_move = move_instruction[1].trim();
                let num: i32 = trimmed_move.parse().expect("Failed to parse string to i32");

                current_position += num * direction;

                while current_position < 0 {
                    current_position += 100;
                }
                while current_position > 99 {
                    current_position -= 100;
                }

                if current_position == 0 {
                    count += 1;
                }

            }
            return count;
        }
        Err(_) => {
            println!("Day 1: Error opening file");
            return 0;
        }
    }
}