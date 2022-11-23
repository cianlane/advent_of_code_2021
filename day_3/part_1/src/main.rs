use std::fs;

fn main() {
    let mut counters: [i32; 12] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    for line in fs::read_to_string("day_3/part_1/input.txt")
            .expect("Should have been able to read the file")
            .lines() {
 
        let mut i: usize = 0;
        for c in line.chars() {
            match c {
                '0' => counters[i] -= 1,
                '1' => counters[i] += 1,
                _ => panic!("Unexpected binary value {} in {}", c, line)
            }
            i += 1
        }
    }

    println!("counters: {:?}", counters);
    let mut gamma: u32 = 0;
    let mut epsilon: u32 = 0;
    let mut i: u32 = 0;
    for num in counters.iter().rev() {
        if num > &0 { gamma += 2_u32.pow(i); }
        else if num < &0 { epsilon += 2_u32.pow(i); }
        else { panic!("I didn't expect it to be zero :("); }
        i += 1
    }

    println!("gamma: {}, epsilon: {}, result: {}", gamma, epsilon, gamma * epsilon);
}
