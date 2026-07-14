use std::{ env::args, fs::read, str::from_utf8 };

fn main() {
    let argv: Vec<String> = args().collect();
    let input_filename = if argv.len() < 2
        { "input.txt" } else { &argv[1] };

    let input_file = read(&input_filename)
        .expect(
            format!("] Couldn't read input file '{}'", &input_filename).as_str()
        );

    let mut value: u8 = 50; // specified initial value
    let mut password: u64 = 0;
    let instructions = from_utf8(&input_file)
        .expect("] Couldn't extract input instructions")
        .lines();

    for instruction in instructions {
        process_instruction(instruction, &mut value);
        println!("Rotated `{}`. Now points at `{}`", instruction, value);

        if value == 0 {
            password += 1;
        }
    }

    println!("> Password: {}", password);
}

/// @param instruction: to be processed
/// @param value: reference to the the mechanism's value
fn process_instruction(instruction: &str, value: &mut u8) {
    let mv_value: i64 = instruction[1..].parse().unwrap();

    move_value(value,
        if instruction.chars().nth(0).unwrap() == 'L' { -mv_value } else { mv_value }
    );
}

fn move_value(value: &mut u8, move_value: i64) {
    let mut value_tmp: i16 = *value as i16;
    if move_value >= 0 {
        value_tmp = ((value_tmp as i64 + move_value) % 100) as i16;
    } else {
        value_tmp = (value_tmp as i64 + move_value % 100) as i16;
        if value_tmp < 0 {
            value_tmp = 100 + value_tmp;
        }
    }

    *value = value_tmp as u8;
}
