use std::{fs, collections::VecDeque};

fn main() {
	let mut window: VecDeque<u32> = VecDeque::new();
	let mut increase_count: u32 = 0;
	for val in fs::read_to_string("day_1/part_2/input.txt")
			.expect("Should have been able to read the file")
			.lines()
			.map(|x: &str| x.parse::<u32>().expect("File should contain only unsigned ints").clone()) {

		match window.len() {
			0..=2 => { window.push_back(val); },
			_ => {
				let prev: u32 = window.pop_front().expect("We should be able to remove the value");
				if val > prev {
					increase_count += 1;
				}
				window.push_back(val);
			}
		}
	}

	println!("{}", increase_count);
}
