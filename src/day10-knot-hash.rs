mod lib;

fn solve(nb_elements: i32, input: &str) -> i32 {
    let input = input.trim().split(",").map(lib::parse_i32);
    let mut elts = (0..nb_elements).collect::<Vec<_>>();
    lib::permute(&mut elts, (0,0), input);
    elts.iter().take(2).cloned().product()
}

fn solve2(input: &str) -> Option<String> {
    String::from_utf8(lib::knot_hash(input.trim().as_bytes())).ok()
}

fn main() {
    let input = lib::read_input_file().unwrap();
    println!("{}", solve(256, &input));
    println!("{}", solve2(&input).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_pb1() {
        assert_eq!(12, solve(5, "3,4,1,5\n\n"));
    }
}
