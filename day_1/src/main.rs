use lazy_static::lazy_static;

lazy_static! {
    static ref DIGITS_STRING: Vec<&'static str> =
        vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    static ref DIGITS_STRING_REVERSED: Vec<&'static str> =
        vec!["eno", "owt", "eerht", "ruof", "evif", "xis", "neves", "thgie", "enin"];
}

fn main() {
    let file_str = include_str!("../input.txt");
    let mut file_vec = file_str.split('\n').collect::<Vec<&str>>();
    file_vec.pop();
    let _: Vec<i32> = file_vec
        .iter()
        .map(|&s| {
            let mut first = 0;
            let mut last = 0;

            let mut index_position = 0;

            for (i, c) in (0_i32..).zip(s.chars()) {
                let n = c as u8;
                if (48..=57).contains(&n) && first == 0 {
                    first = n;
                    index_position = i;

                    break;
                }
            }

            for (j, c) in (0_i32..).zip(
                s.chars()
                    .collect::<Vec<_>>()
                    .into_iter()
                    .rev()
                    .collect::<Vec<_>>()
                    .into_iter(),
            ) {
                let n = c as u8;
                if (48..=57).contains(&n) && last == 0 {
                    if j == s.len() as i32 - index_position {
                        break;
                    }
                    last = n;
                    break;
                }
            }

            if last == 0 {
                let result = char::from(first);
                return format!("{}", result).parse::<i32>().unwrap();
            }

            println!(
                "first: {:?}\nlast:{:?}\n",
                char::from(first),
                char::from(last)
            );

            let result = format!("{}{}", char::from(first), char::from(last));
            result.parse::<i32>().unwrap()
        })
        .collect();
}
