use std::{ env::args, fs::read, str::from_utf8 };

const DIAL_SIZE: usize = 100;

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
        process_instruction(instruction, &mut value, &mut password);
        println!("Rotated `{}`. Now points at `{}`", instruction, value);

        if value == 0 {
            password += 1;
        }
    }

    println!("> Password: {}", password);
}

/// @param instruction: to be processed
/// @param value: reference to the the mechanism's value
fn process_instruction(instruction: &str, value: &mut u8, zeros: &mut u64) {
    let mv_value: i64 = instruction[1..].parse().unwrap();

    update_dial(
        value,
        zeros,
        if instruction.chars().nth(0).unwrap() == 'L' { -mv_value } else { mv_value },
    );
}

fn move_value(value: &mut u8, zeros: &mut u64, mv_value: i64) {
    let mut value_tmp: i16 = *value as i16;
    let mut new_zeros: u64 = 0;
    if mv_value >= 0 {
        new_zeros += ((value_tmp as i64 + mv_value) / DIAL_SIZE as i64) as u64;
        value_tmp = ((value_tmp as i64 + mv_value) % DIAL_SIZE as i64) as i16;
    } else {
        new_zeros += (mv_value / DIAL_SIZE as i64).abs() as u64 +
            if (mv_value % 100).abs() > *value as i64 { 1 } else { 0 };
        value_tmp = (value_tmp as i64 + mv_value % DIAL_SIZE as i64) as i16;
        if value_tmp < 0 {
            value_tmp = DIAL_SIZE as i16 + value_tmp;
        }
    }

    if new_zeros > 0 {
        if *value == 0 { new_zeros -= 1; }
        *zeros += new_zeros;
        println!("Passed {} ZERO", new_zeros);
    }

    *value = value_tmp as u8;
}

fn update_dial(value: &mut u8, zeros: &mut u64, mv_value: i64) {
    let mut value_tmp: i64 = if mv_value >= 0 {
        ((*value as i64 + mv_value) % DIAL_SIZE as i64)
    } else {
        (*value as i64 + mv_value % DIAL_SIZE as i64)
    };

    if value_tmp < 0 && mv_value < 0 {
        value_tmp += DIAL_SIZE as i64;
    }

    println!("vt{}", value_tmp);

    *value = value_tmp as u8;
}
