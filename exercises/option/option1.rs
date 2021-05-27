// option1.rs
// Make me compile! Execute `rustlings hint option1` for hints

// you can modify anything EXCEPT for this function's sig
fn print_number(maybe_number: Option<u16>) {
    println!("printing: {}", maybe_number.unwrap());
}

fn main() {
    print_number(Some(13));
    print_number(Some(99));

    let mut numbers: [Option<u16>; 5] = [Some(3);5];
    /* let mut numbers: Vec<Option<u16>> = Vec::new();
    numbers.push(Some(2));
    numbers.push(Some(3));
    numbers.push(Some(4));
    numbers.push(Some(5));
    numbers.push(Some(6)); */
    for iter in 0..5 {
        let number_to_add: Option<u16> = {
            Some(((iter * 1235) + 2) / (4 * 16))
        };

        numbers[iter as usize] = number_to_add;
    }
}
