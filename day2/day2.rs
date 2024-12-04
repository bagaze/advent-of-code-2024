use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let filename = "./input.txt";
    let mut data: Vec<Vec<i16>> = vec![];

    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {
            data.push(line.split(" ").map(|x| x.parse::<i16>().unwrap()).collect());
        }
    }

    let mut res1: i16 = 0;
    for v in data.iter() {
        res1 += p1_process_vector(&v);
    }
    println!("res - part1: {}", res1);

    let mut res2: i16 = 0;
    for v in data.iter() {
        let mut slices: Vec<Vec<i16>> = vec![];
        for idx in 0..v.len() {
            let mut v_clone = v.clone();
            v_clone.remove(idx);
            slices.push(v_clone);
        }

        res2 += slices.iter().map(|slice| p1_process_vector(&slice)).any(|res| res == 1) as i16;
    }
    println!("res - part2: {}", res2);
}

fn lower_than(x: i16, y: i16) -> bool {
    return x < y;
}

fn more_than(x: i16, y: i16) -> bool {
    return x > y;
}

fn p1_process_vector(v: &[i16]) -> i16 {
    let op: fn(i16, i16) -> bool;

    if v[0] > v[1] {
        op = more_than;
    } else if v[0] < v[1] {
        op = lower_than;
    } else {
        return 0;
    }

    for n in 0..v.len() - 1 {
        let diff: i16 = (v[n + 1] - v[n]).abs();
        if !op(v[n], v[n + 1]) || diff < 1 || diff > 3 {
            return 0;
        }
    }

    return 1;
}

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

