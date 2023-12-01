use std::io::Read;
use std::fs::File;

const BUFFER_SIZE: usize = 102400;
const DOGSHIT_CONVERSION: i32 = 48;

fn main() {
    println!("Hello, world!");
    let calibration_value = decode_calibration(String::from("calibration"));
    println!("{calibration_value}");
}

fn decode_calibration (path: String) -> i32 {
    let mut calibration_value: i32 = 0;
    let mut buffer = [0u8; BUFFER_SIZE];
    let mut source_file = File::open(path).unwrap();
    let mut line_value = Vec::new();

        source_file.read(&mut buffer).unwrap();
        for i in 0..BUFFER_SIZE {
            let char = buffer[i] as char;
            if char.is_numeric() {
                line_value.push(char as i32 - DOGSHIT_CONVERSION);
            } 
            if char == '\n' {
                let first = 0; let last = line_value.len() - 1;
                calibration_value += line_value[first] * 10 + line_value[last];
                line_value.clear();
            }
        }

    calibration_value
}

/*
fn decode_calibration_with_buffer (path: String) -> i32 {
    let mut calibration_value: i32 = 0;
    let mut buffer = [0u8; BUFFER_SIZE];
    let mut source_file = File::open(path).unwrap();
    let mut line_value = Vec::new();

    loop {
        let read_count = source_file.read(&mut buffer).unwrap();
        for i in 0..BUFFER_SIZE {
            let char = buffer[i] as char;
            if char.is_numeric() {
                line_value.push(char as i32 - DOGSHIT_CONVERSION);
            } 
            if char == '\n' {
                let first = 0; let last = line_value.len() - 1;
                calibration_value += line_value[first] * 10 + line_value[last];
                line_value.clear();
            }
        }

        if read_count < BUFFER_SIZE {
            break;
        }
    }
    calibration_value
}
*/
