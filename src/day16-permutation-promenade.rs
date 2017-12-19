mod lib;

#[derive(Clone)]
enum Move {
    Spin(usize),
    Exchange(usize, usize),
    Partner(u8, u8),
}

fn usize(input: &[u8], lo: usize) -> Option<(usize, usize)> {
    fn is_digit(b: u8) -> bool {
        match b {
            b'0'...b'9' => true,
            _ => false,
        }
    };
    let mut hi = lo;
    while hi < input.len() && is_digit(input[hi]) {
        hi += 1;
    }
    std::str::from_utf8(&input[lo..hi]).ok().and_then(|str| {
        str.parse().ok().map(|res| (hi,res))
    })
}

fn update(mut st: (Vec<u8>, usize), item: Move) -> (Vec<u8>, usize) {
    match item {
        Move::Spin(s) => {
            let len = st.0.len();
            (st.0, (len + st.1 - s) % len)
        },
        Move::Exchange(i,j) => {
            let len = st.0.len();
            st.0.swap((i+st.1) % len, (j+st.1) % len);
            st
        },
        Move::Partner(a,b) => {
            let ia = st.0.iter().position(|&e| e == a).unwrap();
            let ib = st.0.iter().position(|&e| e == b).unwrap();
            st.0.swap(ia,ib);
            st
        },
    }
}

fn reorder((st, ofs): (Vec<u8>, usize)) -> Option<String> {
    let mut res = std::iter::repeat(b'0').take(st.len()).collect::<Vec<_>>();
    res[..st.len()-ofs].clone_from_slice(&st[ofs..]);
    res[st.len()-ofs..].clone_from_slice(&st[..ofs]);
    String::from_utf8(res).ok()
}

fn parse_line(line: &str) -> Option<Move> {
    let line = line.as_bytes();
    match line[0] {
        b's' => usize(line, 1).and_then(|(hi, x)| {
            if hi == line.len() {
                Some(Move::Spin(x))
            } else {
                None
            }
        }),
        b'x' => usize(line, 1).and_then(|(hi, x1)| {
            if line[hi] == b'/' {
                usize(line, hi+1).and_then(|(hi, x2)| {
                    if hi == line.len() {
                        Some(Move::Exchange(x1,x2))
                    } else {
                        None
                    }
                })
            } else {
                None
            }
        }),
        b'p' => {
            if line.len() == 4 && line[2] == b'/' {
                Some(Move::Partner(line[1], line[3]))
            } else {
                None
            }
        },
        _ => None,
    }
}

fn init(nb_chars: usize) -> (Vec<u8>, usize) {
    ((b'a'..).take(nb_chars).collect(), 0)
}

fn solve<Iter: Iterator<Item = Move>>(nb_chars: usize, input: Iter) -> Option<String> {
    reorder(input.fold(init(nb_chars), update))
}

fn solve2(nb_chars: usize, input: Vec<Move>) -> Option<String> {
    let mut st = init(nb_chars);
    let mut past_states = vec![];
    loop {
        let cur_st = reorder(st.clone())?;
        match past_states.iter().position(|ref x| **x == cur_st) {
            Some(_) => break,
            None => past_states.push(cur_st),
        };
        st = input.iter().cloned().fold(st, update);
    }
    let len = past_states.len();
    Some(std::mem::replace(&mut past_states[1_000_000_000 % len], "".to_owned()))
}

fn main() {
    let input = &lib::read_input_file().unwrap();
    let input = input.trim().split(",").map(|line| parse_line(line).unwrap()).collect::<Vec<_>>();
    println!("{}", solve(16, input.iter().cloned()).unwrap());
    println!("{}", solve2(16, input).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!("baedc", &solve(5, vec![Move::Spin(1), Move::Exchange(3,4), Move::Partner(b'e', b'b')].iter().cloned()).unwrap())
    }
}
