use std::{fs};

fn from_binary_string(str: &String) -> u32
{
	let mut val: u32 = 0;
	for (i, c) in str.chars().rev().enumerate() {
		if c == '1'{
			val += 2_u32.pow(i.try_into().unwrap());
		}
	}
	return val;
}

fn determine_dominant_value(strings: &Vec<String>, index: usize) -> i32
{
	let mut res: i32 = 0;
	for s in strings.iter() {
		match s.chars().nth(index).unwrap() {
			'0' => res -= 1,
			'1' => res+= 1,
			_ => panic!("Unexpected non-binary value in {}", s)
		}
	}

	return res;
}

fn main() {
	let input: String = fs::read_to_string("day_3/part_2/input.txt")
			.expect("Should have been able to read the file");
	let lines: Vec<String> = input.lines().map(|x| String::from(x)).collect();

	let mut oxygen_lines = lines.clone();
	let mut index: usize = 0;
	while oxygen_lines.len() > 1
	{
		let val = determine_dominant_value(&oxygen_lines, index);
		let mut remaining_lines = oxygen_lines.len();
		oxygen_lines = oxygen_lines.iter()
			.filter(|x| -> bool {
				if remaining_lines == 1 {
					return true;
				}
				if val >= 0 && x.chars().nth(index).unwrap() == '1' {
					return true;
				}
				else if val < 0 && x.chars().nth(index).unwrap() == '0'
				{
					return true;
				}
				remaining_lines -= 1;
				return false;
			})
			.map(|x: &String| String::from(x))
			.collect();
		index += 1;
	}

	assert!(oxygen_lines.len() == 1);
	let oxygen_generator_rating = from_binary_string(oxygen_lines.first().unwrap());
	println!("oxygen generator rating {} -> {}", oxygen_lines.first().unwrap(), oxygen_generator_rating);

	let mut co2_lines = lines.clone();
	let mut index: usize = 0;
	while co2_lines.len() > 1
	{
		let val = determine_dominant_value(&co2_lines, index);
		let mut remaining_lines = co2_lines.len();
		co2_lines = co2_lines.iter()
			.filter(|x| -> bool {
				if remaining_lines == 1 {
					return true;
				}
				if val < 0 && x.chars().nth(index).unwrap() == '1' {
					return true;
				}
				else if val >= 0 && x.chars().nth(index).unwrap() == '0'
				{
					return true;
				}
				remaining_lines -= 1;
				return false;
			})
			.map(|x: &String| String::from(x))
			.collect();
		index += 1;
	}

	assert!(co2_lines.len() == 1);
	let co2_scrubber_rating = from_binary_string(co2_lines.first().unwrap());
	println!("co2 scrubber rating {} -> {}", co2_lines.first().unwrap(), co2_scrubber_rating);

	println!("result: {}", oxygen_generator_rating * co2_scrubber_rating);
}
