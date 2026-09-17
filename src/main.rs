use std::io::{Read, stdin};

fn main() {
    let mut input = String::new();

    stdin().read_to_string(&mut input).unwrap();

    let mut result: u128 = 0;
    for line in input.split('\n') {
        if line.len() == 0 {
            continue;
        }
        let line = line.trim();
        if line == "-1" {
            break;
        }
        let output = line.parse::<u128>();
        match output {
            Ok(val) => {
                result += val;
            }
            Err(_) => {
                print!("NaN");
                return;
            }
        }
    }
    print!("{result}");
}
