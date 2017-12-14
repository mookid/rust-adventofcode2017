mod lib;

struct State {
    cur_pos: i32,
    skip_size: i32,
}

fn permute<Iter: Iterator<Item = i32>>(elts: &mut Vec<i32>, st: State, input: Iter) -> State {
    let nb_elements = elts.len() as i32;
    input.fold(st, |State {cur_pos, skip_size}, len| {
        assert!(0 <= cur_pos);
        let mut lo = cur_pos;
        let mut hi = cur_pos + len - 1;
        while lo < hi {
            elts.swap((lo % nb_elements) as usize, (hi % nb_elements) as usize );
            lo += 1;
            hi -= 1;
        }
        State {
            cur_pos: cur_pos + len + skip_size as i32,
            skip_size: skip_size + 1
        }
    })
}

fn solve(nb_elements: i32, input: &str) -> i32 {
    let input = input.trim().split(",").map(lib::parse_i32);
    let mut elts :Vec<_> = (0..nb_elements).collect();
    let st = State {
        cur_pos: 0,
        skip_size: 0,
    };
    permute(&mut elts, st, input);
    elts.iter().take(2).cloned().product()
}

fn solve2(input: &str) -> String {
    String::from_utf8(lib::knot_hash(input.trim().as_bytes())).unwrap()
}

fn main() {
    let input = lib::read_input_file().unwrap();
    println!("{}", solve(256, &input));
    println!("{}", solve2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_pb1() {
        assert_eq!(12, solve(5, "3,4,1,5\n\n"));
    }
}
