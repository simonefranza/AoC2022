use std::fs::File;
use std::io::Read;
use std::vec::Vec;

fn main() -> std::io::Result<()> {
    let mut file = File::open("./src/input")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let mut values : Vec<u64> = Vec::new();
    values.push(0);
    let lines = contents.split('\n');
    for line in lines {
        if line == "" {
            values.push(0);
            continue;
        }
        let len = values.len().saturating_sub(1);
        values[len] += line.parse::<u64>().unwrap();
    }
    let mut max_value = 0;
    let mut max_value2 = 0;
    let mut max_value3 = 0;
    max_value += values.iter().max().unwrap();
    // Part 1
    println!("Max {}", max_value);

    // Part 2
    values.remove(values.iter().position(|x| *x == max_value).expect("needle not found"));
    max_value2 += values.iter().max().unwrap();
    values.remove(values.iter().position(|x| *x == max_value2).expect("needle not found"));
    max_value3 += values.iter().max().unwrap();
    println!("Sum {}", max_value + max_value2 + max_value3);

    Ok(())
}
