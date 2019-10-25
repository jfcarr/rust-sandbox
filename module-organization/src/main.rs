mod lib;
use crate::lib::math as mymath;
use crate::lib::trig as mytrig;

fn main() {
	let first_number: f64 = 5.0;
	let second_number: f64 = 10.0;

	println!(
		"Tan of {} is {}",
		first_number,
		mymath::add_two_numbers(first_number, second_number)
	);

	// Access the trig::tan() function via the math module like this:
	println!(
		"Tan of {} is {}",
		first_number,
		mymath::get_tan(first_number)
	);

	// ..or directly like this:
	println!("Tan of {} is {}", first_number, mytrig::tan(first_number));
}
