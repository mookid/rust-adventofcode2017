#![allow(dead_code)]

use ::std::io::Read;

pub fn read_input_file() -> Result<String, Box<::std::error::Error>> {
    let input_file = ::std::env::args().nth(1).ok_or("usage")?;
    let mut file = ::std::fs::File::open(input_file)?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;
    Ok(input)
}

pub fn parse_i32(str: &str) -> i32 {
    str.parse::<i32>().unwrap()
}

pub fn parse_from<'a, T : Copy>(table: &'a [(&'static str, T)], input: &'a str) -> T {
    table.iter().find(|&&(key, _)| key == input)
        .map(|&(_, value)| value)
        .expect(("parse_from: ".to_owned() + &input.to_owned()).as_str())
}

fn hexa2(n:i32) -> Vec<u8> {
    [n / 16, n % 16].iter().cloned().map(hexa).collect()
}

fn sparse_to_dense(sparse: Vec<i32>) -> Vec<u8> {
    let mut dense = vec![b'0'; sparse.len()/8];
    dense.chunks_mut(2).zip(sparse.chunks(16))
        .for_each(|(dense, sparse)| {
            dense.clone_from_slice(&hexa2(
                sparse.iter().cloned().fold(0, |acc, x| acc^x)))
        });
    dense
}

fn hexa(n: i32) -> u8 {
    match n {
        0 => b'0', 1 => b'1', 2 => b'2', 3 => b'3',
        4 => b'4', 5 => b'5', 6 => b'6', 7 => b'7',
        8 => b'8', 9 => b'9', 10 => b'a', 11 => b'b',
        12 => b'c', 13 => b'd', 14 => b'e', 15 => b'f',
        _ => panic!("hexa"),
    }
}

pub fn permute<Iter: Iterator<Item = i32>>(elts: &mut Vec<i32>, st: (i32,i32), input: Iter) -> (i32,i32) {
    let nb_elements = elts.len() as i32;
    input.fold(st, |(cur_pos, skip_size), len| {
        assert!(0 <= cur_pos);
        let mut lo = cur_pos;
        let mut hi = cur_pos + len - 1;
        while lo < hi {
            elts.swap((lo % nb_elements) as usize, (hi % nb_elements) as usize );
            lo += 1;
            hi -= 1;
        }
        (cur_pos + len + skip_size as i32, skip_size + 1)
    })
}

pub fn knot_hash(input: &[u8]) -> Vec<u8> {
    let mut elts = (0..256).collect::<Vec<_>>();
    let mut input = input.iter().cloned().map(|b| b as i32).collect::<Vec<_>>();
    input.extend_from_slice(&[17, 31, 73, 47, 23]);
    (0..64).fold((0,0), |st,_| permute(&mut elts, st, input.iter().cloned()));
    sparse_to_dense(elts)
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_knot_hash() {
        assert_eq!(b"a2582a3a0e66e6e86e3812dcb672a272", &knot_hash(b"")[..]);
        assert_eq!(b"33efeb34ea91902bb2f59c9920caa6cd", &knot_hash(b"AoC 2017")[..]);
        assert_eq!(b"3efbe78a8d82f29979031a4aa0b16a9d", &knot_hash(b"1,2,3")[..]);
        assert_eq!(b"63960835bcdc130f0b66d7ff4f6a5a8e", &knot_hash(b"1,2,4")[..]);
    }
}
