fn main() {
    let file_str = include_str!("../input.txt");
    let games = file_str
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|s| Some(s.split(':')));

    let mut acc: i32 = 0;

    for g in games {
        let mut sets: Vec<&str> = Vec::new();

        let mut zeroed_vector: Vec<i32> = vec![0; 3];
        let mins: &mut Vec<i32> = &mut zeroed_vector;

        sets.extend(g.unwrap().skip(1).flat_map(|s| s.trim().split(';')));
        for c in sets.iter() {
            let single_set = c.trim().split(',').collect::<Vec<&str>>();
            is_possible(single_set, mins);
        }
        acc += mins[0] * mins[1] * mins[2];
    }
    println!("{:?}", acc);
}

fn is_possible(set: Vec<&str>, mins: &mut Vec<i32>) {
    println!("evaluating set: {:?}", set);
    let mut counts = [0; 3];

    for s in set {
        let parts: Vec<&str> = s.split_whitespace().collect();
        let count = parts[0].parse::<i32>().unwrap();

        match parts[1] {
            "red" => {
                if count > mins[0] {
                    mins[0] = count;
                    counts[0] += count;
                }
            }
            "green" => {
                if count > mins[1] {
                    mins[1] = count;
                    counts[1] += count;
                }
            }
            "blue" => {
                if count > mins[2] {
                    mins[2] = count;
                    counts[2] += count;
                }
            }
            _ => panic!("Invalid color"),
        }
    }
}
