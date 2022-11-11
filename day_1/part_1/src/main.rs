use std::fs;

fn main() {
    let mut prev: Option<u32> = None;
    let mut increase_count: u32 = 0;
    for val in fs::read_to_string("input.txt")
            .expect("Should have been able to read the file")
            .lines()
            .map(|x: &str| x.parse::<u32>().expect("File should contain only unsigned ints").clone()) {
        match prev {
            Some(p) => if val > p { increase_count +=1; },
            None => {}
        }
        prev = Some(val)
    }

    println!("{}", increase_count);
}
