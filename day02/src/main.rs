use std::{fs::File, io::Read};

fn evaluate_match(a : char, b : char) -> u32 {
    let norm_a = a as u8 - 'A' as u8;
    let norm_b = b as u8 - 'X' as u8;
    let mut points = norm_b as u32 + 1;
    if norm_a == norm_b {
        points += 3;
    } else if norm_b == ((norm_a + 1) % 3) {
        points += 6;
    }

    return points;
}


fn evaluate_cheat(opponent_move: char, strategy: char) -> u32 {
    let lose = 0;
    let draw = 1;
    let win = 2;
    let norm_opponent = opponent_move as u8 - 'A' as u8;
    let norm_strategy = strategy as u8 - 'X' as u8;
    let mut points : u32 = 1;

    match norm_strategy {
        _ if norm_strategy == draw => points += norm_opponent as u32 + 3,
        _ if norm_strategy == win => points += (norm_opponent as u32 + 1) % 3 + 6,
        _ if norm_strategy == lose => points += (norm_opponent as u32 + 2) % 3,
        _ => println!("error"),
    }

    return points;
}

fn main() -> std::io::Result<()> {
    let mut file = File::open("./src/input")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    let mut points : u64 = 0;
    let mut cheat_points : u64 = 0;
    let lines = content.split('\n');
    for line in lines {
        if line == "" {
            continue;
        }
        let signs : Vec<&str> = line.split(' ').collect();
        points += evaluate_match(signs[0].chars().nth(0).unwrap(), signs[1].chars().nth(0).unwrap()) as u64;
        cheat_points += evaluate_cheat(signs[0].chars().nth(0).unwrap(), signs[1].chars().nth(0).unwrap()) as u64;
    }
    println!("Task1 points {}", points);
    println!("Task2 points {}", cheat_points);

    Ok(())
}
