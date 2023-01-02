use std::io;

fn get_fib_val(place: u128) -> u128 {
    let mut current_place: u128 = 0;
    let mut current_value: u128 = 0;
    let mut values: Vec<u128> = vec![0, 0];

    // 0 1 1 2 3...
    while current_place <= place {
        if current_value == 0 && current_place == 1 {
            current_value += 1;
        } else {
            current_value = values[0] + values[1];
        }
        current_place += 1;
        values[0] = values[1];
        values[1] = current_value;
    };

    return current_value;
}

fn main() {
    println!("What place of the fibonacci sequence would you like to get? ");

    let mut place = String::new();
    io::stdin().read_line(&mut place).expect("Failed to read line.");
    let place: u128 = place.trim().parse().expect("Not a number.");
    let value: u128 = get_fib_val(place);

    println!("The fibonacci sequence value at {} is {}", place, value);
}
