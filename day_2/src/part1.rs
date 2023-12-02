fn main() {
    let file_str = include_str!("../input.txt");
    let games = file_str
        .lines()
        .filter(|s| !s.is_empty())
        .map(|s| Some(s.split(':')));

    let mut sum = 0;
    for (i, g) in games.enumerate() {
        let sets: Vec<&str> = g
            .unwrap()
            .skip(1)
            .flat_map(|s| s.trim().split(','))
            .collect();
        let mut found = true;

        for c in sets.iter() {
            let single_set = c.trim().split(';').collect::<Vec<&str>>();
            if !is_possible(&single_set) {
                found = false;
                break;
            }
        }
        if found {
            sum += i + 1;
        }
    }
    println!("{:?}", sum);
}

fn is_possible(set: &[&str]) -> bool {
    let mut counts = [0; 3];

    for cube in set {
        let parts: Vec<&str> = cube.split_whitespace().collect();
        let count = parts[0].parse::<i32>().unwrap();
        match parts[1] {
            "red" => counts[0] += count,
            "green" => counts[1] += count,
            "blue" => counts[2] += count,
            _ => panic!("Invalid color"),
        }
        if counts[0] >= 12 || counts[1] >= 13 || counts[2] >= 14 {
            return false;
        }
    }
    true
}
