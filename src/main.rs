use std::fs::{read_to_string, write};

fn find_first_number_position(input: &str) -> Option<usize> {
    let mut position = None;
    for (idx, c) in input.chars().enumerate() {
        if c.is_digit(10) {
            position = Some(idx);
            break;
        }
    }
    position
}

fn main() {
    let path = "/home/sprechtl/.config/hypr/transform.conf";
    let input = read_to_string(&path).expect("Could not read transform.conf");
    let index_of_number = find_first_number_position(&input).expect("No number in line");
    let mut output = "".to_string();
    if let Some(tranform) = input.chars().nth(index_of_number) {
        output = input.chars().take(index_of_number).collect::<String>();
        if tranform == '0' {
            output.push('2');
        } else {
            output.push('0');
        }
    } else {
        panic!("Could not find tranform");
    }
    write(path, output).expect("Could not write transform.conf");
}
