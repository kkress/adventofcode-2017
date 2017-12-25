use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn part1() {
    let mut f = File::open("input.txt").unwrap();
    let line_buffer = BufReader::new(&f);

    let cksum: u32 = line_buffer.lines()
        .map(|curr| curr.unwrap().split_whitespace().map(|s| s.parse::<u32>().unwrap()).collect::<Vec<u32>>())
        .map(|l| l.iter().max().unwrap() - l.iter().min().unwrap())
        .sum();

    println!("Chsum = {}", cksum);
}

fn part2() {
    let mut f = File::open("input.txt").unwrap();
    let line_buffer = BufReader::new(&f);
    let mut cksum = 0;

    for line in line_buffer.lines() {
        let curr = line.unwrap();
        println!("{}", curr);
        let dimensions: Vec<u32> = curr.split_whitespace().map(|s| s.parse::<u32>().unwrap()).collect();
        let mut max = 0;
        for i in 0..dimensions.len() {
            let num = dimensions[i];
            for j in 0..dimensions.len() {
                if i == j {
                    continue;
                }
                let denom = dimensions[j];
                if num % denom == 0 {
                    println!("{}/{}", num, denom);
                    max = std::cmp::max(max, num/denom);
                }
            }
        }
        cksum += max;
    }
    println!("Chsum = {}", cksum);
}

fn main() {
    part1();
    part2();
}
