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

fn xorall(it: &[i32]) -> Option<i32> {
    use std::ops::BitXor;
    let mut it = it.iter().cloned();
    it.next().map(|st| it.fold(st, i32::bitxor))
}

fn hexa(n: i32) -> char {
    match n {
        0 => '0',
        1 => '1',
        2 => '2',
        3 => '3',
        4 => '4',
        5 => '5',
        6 => '6',
        7 => '7',
        8 => '8',
        9 => '9',
        10 => 'a',
        11 => 'b',
        12 => 'c',
        13 => 'd',
        14 => 'e',
        15 => 'f',
        _ => panic!("hexa"),
    }
}

fn hexa2(n:i32) -> String {
    vec![n / 16, n % 16].iter().cloned().map(hexa).collect()
}

fn hash(sparse_hash: Vec<i32>) -> String {
    let chunks : Vec<_> = sparse_hash.chunks(16)
        .map(|chunk| xorall(chunk).unwrap())
        .map(hexa2)
        .collect();
    chunks.join("")
}

fn solve2(nb_elements: i32, input: &str) -> String {
    let mut elts :Vec<_> = (0..nb_elements).collect();
    let mut input : Vec<_> = input.trim().as_bytes().iter().cloned().map(|b| b as i32).collect();
    input.append(&mut (vec![17, 31, 73, 47, 23]));
    (0..64).fold(State {
        cur_pos: 0,
        skip_size: 0,
    }, |st,_| permute(&mut elts, st, input.iter().cloned()));
    hash(elts)
}

fn main() {
    let input = lib::read_input_file().unwrap();
    println!("{}", solve(256, &input));
    println!("{}", solve2(256, &input));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_pb1() {
        assert_eq!(12, solve(5, "3,4,1,5\n\n"));
    }
    #[test]
    fn test_hash() {
        assert_eq!("a2582a3a0e66e6e86e3812dcb672a272", solve2(256, ""));
        assert_eq!("33efeb34ea91902bb2f59c9920caa6cd", solve2(256, "AoC 2017"));
        assert_eq!("3efbe78a8d82f29979031a4aa0b16a9d", solve2(256, "1,2,3"));
        assert_eq!("63960835bcdc130f0b66d7ff4f6a5a8e", solve2(256, "1,2,4"));
    }

}
