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

            // Second part
            // let mut tracking_string = "".to_string();
            // let mut digit_position = 0;

            for (i, c) in (0_i32..).zip(s.chars()) {
                let n = c as u8;
                if (48..=57).contains(&n) && first == 0 {
                    first = n;
                    index_position = i;

                    break;
                }

                // Second part
                // let number: String;
                // let continuing: bool;
                // (number, tracking_string, continuing) = discovering_digit_word(
                //     c,
                //     digit_position,
                //     &mut tracking_string,
                //     DIGITS_STRING.to_vec(),
                // );
                // if continuing {
                //     digit_position += 1;
                // }
                // if !number.is_empty() {
                //     first = word_to_number(&number).unwrap();
                //     break;
                // }
            }

            // digit_position = 0;
            // tracking_string = "".to_string();
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

                // Second part
                // let number: String;
                // let mut continuing: bool = false;
                // (number, tracking_string, continuing) = discovering_digit_word(
                //     c,
                //     digit_position,
                //     &mut tracking_string,
                //     DIGITS_STRING_REVERSED.to_vec(),
                // );
                // if continuing {
                //     digit_position += 1;
                // }
                // if !number.is_empty() {
                //     last = word_to_number(&number).unwrap();

                //     break;
                // }
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

// Second part
// fn discovering_digit_word(
//     c: char,
//     i: usize,
//     tracking_string: &mut String,
//     vector: Vec<&'static str>,
// ) -> (String, String, bool) {
//     let mut ts: String = tracking_string.to_owned();
//     for &d in vector.iter() {
//         if i >= d.len() {
//             continue;
//         }
//         if i == d.len() - 1 && char::from(d.as_bytes()[i]) == c && d.contains(&ts) {
//             ts.push(c);

//             return (d.to_string(), ts, true);
//         }
//         if char::from(d.as_bytes()[i]) == c && d.contains(&ts) {
//             ts.push(c);
//             return ("".to_string(), ts, true);
//         }
//     }
//     ("".to_string(), "".to_string(), false)
// }

// fn word_to_number(word: &str) -> Option<u8> {
//     match word.to_lowercase().as_str() {
//         "eno" => Some(b'1'),
//         "owt" => Some(b'2'),
//         "eerht" => Some(b'3'),
//         "ruof" => Some(b'4'),
//         "evif" => Some(b'5'),
//         "xis" => Some(b'6'),
//         "neves" => Some(b'7'),
//         "thgie" => Some(b'8'),
//         "enin" => Some(b'9'),
//         "one" => Some(b'1'),
//         "two" => Some(b'2'),
//         "three" => Some(b'3'),
//         "four" => Some(b'4'),
//         "five" => Some(b'5'),
//         "six" => Some(b'6'),
//         "seven" => Some(b'7'),
//         "eight" => Some(b'8'),
//         "nine" => Some(b'9'),
//         _ => None,
//     }
// }
