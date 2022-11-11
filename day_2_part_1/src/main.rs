use std::fs;

fn main() {
    let mut depth: i32 = 0;
    let mut position: i32 = 0;
    // let mut prev: Option<u32> = None;
    // let mut increase_count: u32 = 0;
    for val in fs::read_to_string("input.txt")
            .expect("Should have been able to read the file")
            .lines()
            .map(|x: &str| {
                let mut split = x.split(" ").clone();
                let command: &str = split.next().expect("Command expected but not found");
                let distance: i32 = split.next().expect("Distance expected but not found")
                        .parse::<i32>().expect("Expect the distance to be a u32");
                (command, distance)
            }) {
        match val {
            ("up", _) => depth -= val.1,
            ("down", _) => depth += val.1,
            ("forward", _) => position += val.1,
            _ => panic!("unexpected command")
        }
    }

    println!("depth: {}, position: {}, result: {}", depth, position, depth * position);
}
