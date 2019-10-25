use crate::lib::trig;

pub fn add_two_numbers(first_number: f64, second_number: f64) -> f64 {
	return first_number + second_number;
}

pub fn get_tan(input: f64) -> f64 {
	return trig::tan(input);
}
