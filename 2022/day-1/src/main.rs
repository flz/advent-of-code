use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let f = File::open("input")?;
    let mut bufreader = BufReader::new(f);

    let mut acc = 0;
    let mut sums = Vec::<i32>::new();

    loop {
        let mut line = String::new();

        if bufreader.read_line(&mut line)? == 0 {
            sums.push(acc);
            break
        }

        let clean = line.trim();

        if clean.is_empty() {
            sums.push(acc);
            acc = 0;
        } else {
            acc += clean.parse::<i32>().unwrap();
        }
    }

    sums.sort();
    let len = sums.len();

    println!("{}", sums[len-1]);
    println!("{}", &sums[len-3..].iter().sum::<i32>());

    Ok(())
}
