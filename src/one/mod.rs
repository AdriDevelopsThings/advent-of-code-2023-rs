fn get_first_last_digit(str: &str) -> (u32, u32) {
    let mut first_num: Option<u32> = None;
    let mut last_num: Option<u32> = None;
    for c in str.chars() {
        match c.to_string().parse::<u32>() {
            Ok(n) => {
                if first_num.is_none() {
                    first_num = Some(n);
                }
                last_num = Some(n);
            }
            Err(_) => continue,
        }
    }
    (first_num.unwrap(), last_num.unwrap())
}

pub fn solve_first(input: String) -> String {
    input
        .lines()
        .map(|line| {
            let (first, last) = get_first_last_digit(line);
            first * 10 + last
        })
        .sum::<u32>()
        .to_string()
}

fn get_first_last_digit_second(str: &str) -> (u32, u32) {
    let mut first: Option<u32> = None;
    let mut last: Option<u32> = None;

    for i in 0..str.len() {
        let substr = &str[i..];
        let n = match substr[0..1].parse::<u32>() {
            Ok(n) => n,
            Err(_) => {
                if substr.starts_with("zero") {
                    0
                } else if substr.starts_with("one") {
                    1
                } else if substr.starts_with("two") {
                    2
                } else if substr.starts_with("three") {
                    3
                } else if substr.starts_with("four") {
                    4
                } else if substr.starts_with("five") {
                    5
                } else if substr.starts_with("six") {
                    6
                } else if substr.starts_with("seven") {
                    7
                } else if substr.starts_with("eight") {
                    8
                } else if substr.starts_with("nine") {
                    9
                } else {
                    continue;
                }
            }
        };
        if first.is_none() {
            first = Some(n);
        }
        last = Some(n);
    }
    (first.unwrap(), last.unwrap())
}

pub fn solve_second(input: String) -> String {
    input
        .lines()
        .map(|line| {
            let (first, last) = get_first_last_digit_second(line);
            first * 10 + last
        })
        .sum::<u32>()
        .to_string()
}
