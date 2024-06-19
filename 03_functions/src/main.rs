// Parameters function: specify the param name and type.
fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

// Return function: whenever you do not put ; 
// in a statement it will turn into a expression returning it last value.
fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

// Main function: first function to be executed in a rust program.
fn main() {
    another_function(5);
    print_labeled_measurement(5, 'h');
    let my_number_five = plus_one(five());
    println!("The function has returned: {my_number_five}");
}
