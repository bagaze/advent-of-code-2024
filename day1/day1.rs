use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut v1: Vec<i32> = Vec::new();
    let mut v2: Vec<i32> = Vec::new();

    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {
            let mut line_split = line.split("   ");
            let val1 = line_split.next().unwrap();
            v1.push(val1.parse::<i32>().unwrap());
            let val2 = line_split.next().unwrap();
            v2.push(val2.parse::<i32>().unwrap());
        }
    }
    v1.sort();
    v2.sort();

    println!("p1: {}", p1_distance_vectors(&v1, &v2));
    println!("p1: {}", p2_count_simmilarities(&v1, &v2));
}

fn p1_distance_vectors(v1: &[i32], v2: &[i32]) -> i32 {
    let mut res: i32 = 0;

    for n in 0..v1.len() {
        if v1[n] > v2[n] {
            res += v1[n] - v2[n]
        } else if v1[n] < v2[n] {
            res += v2[n] - v1[n]
        }
    }

    return res
}

fn p2_count_simmilarities(v1: &[i32], v2: &[i32]) -> i32 {
    let mut res: i32 = 0;

    for n1 in v1.iter() {
        let mut count: i32 = 0;

        for n2 in v2.iter() {
            if n1 == n2 {
                count += 1;
            } else if n2 > n1 {
                break;
            }
        }
        res += n1 * count;
    }

    return res;
}

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
