use std::fs::File;
use std::io::{self, prelude::*, BufReader};

/// Input: mass
/// Returns: fuel, counting the fuel required for the fuel itself
fn fuel(mass: isize) -> isize {
    (mass/3)-2
}

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut totfuel = 0;
    for line in reader.lines() {
        let mass = line.unwrap().parse().unwrap();
        totfuel += fuel(mass);
    }
    println!("fuel: {}", totfuel);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn fuel_examples() {
        assert_eq!(fuel(12), 2);
        assert_eq!(fuel(14), 2);
        assert_eq!(fuel(1969), 654);
        assert_eq!(fuel(100756), 33583);
    }
}
