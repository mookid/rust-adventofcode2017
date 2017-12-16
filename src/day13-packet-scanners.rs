mod lib;

fn parse(str: &str) -> Option<Vec<(i32,i32)>> {
    str.trim().lines()
        .map(|s| {
            let mut it = s.split(|s| s == ':' || char::is_whitespace(s))
                .filter(|&s| s != "");
            let fst = lib::parse_i32(it.next()?);
            let snd = lib::parse_i32(it.next()?);
            Some((fst, snd))
        })
        .collect()
}

fn caught(delay: i32, depth: i32, range: i32) -> bool {
    range == 1 || (depth + delay) % (2*range-2) == 0
}

fn total_severity(input: &Vec<(i32,i32)>) -> i32 {
    input.iter()
        .filter_map(|&(depth,range)| {
            if caught(0, depth, range) {
                Some(depth*range)
            } else {
                None
            }
        })
        .sum()
}

fn find_best_delay(input: &Vec<(i32,i32)>) -> i32 {
    for t in 0.. {
        !input.iter().any(|&(depth,range)| caught(t, depth, range)) && return t;
    }
    panic!()
}

fn main() {
    let input = parse(&lib::read_input_file().unwrap()).unwrap();
    println!("{:?}", total_severity(&input));
    println!("{:?}", find_best_delay(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        let str = r#"
0: 3
1: 2
4: 4
6: 4
"#;
        let input = parse(str).unwrap();
        assert_eq!(total_severity(&input), 24);
        assert_eq!(find_best_delay(&input), 10);
    }
}
