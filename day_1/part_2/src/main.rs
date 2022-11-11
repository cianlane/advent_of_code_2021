extern crate queues;
use queues::*;
use std::fs;

fn main() {
	let mut window: Queue<u32> = queue![];
	let mut increase_count: u32 = 0;
	for val in fs::read_to_string("input.txt")
			.expect("Should have been able to read the file")
			.lines()
			.map(|x: &str| x.parse::<u32>().expect("File should contain only unsigned ints").clone()) {

		match window.size() {
			0..=2 => { window.add(val).expect("We should be able to add the value"); },
			_ => {
				let prev: u32 = window.remove().expect("We should be able to remove the value");
				if val > prev {
					increase_count += 1;
				}
				window.add(val).expect("We should be able to add the value");
			}
		}
	}

	println!("{}", increase_count);
}
