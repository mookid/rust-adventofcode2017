mod lib;

const MASK : i32 = 1<<16;

fn parse(str: &str) -> Option<(i32,i32)> {
    let mut seeds = str.lines()
        .map(|line| line
             .split(char::is_whitespace)
             .nth(4)
             .map(lib::parse_i32));
    let fst = seeds.next()??;
    let snd = seeds.next()??;
    Some((fst,snd))
}

struct Seq {
    factor: i32,
    cur: i32,
}

impl<'a> Iterator for Seq {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        let factor = self.factor as u64;
        let cur = self.cur as u64;
        let cur = ((factor * cur) % 2147483647) as i32;
        self.cur = cur;
        Some(cur)
    }
}

fn new_seq(factor: i32, init: i32) -> Seq {
    Seq {
        factor: factor,
        cur: init,
    }
}

fn compare_seqs<Iter1, Iter2>(seq1: Iter1, seq2: Iter2, nb_pairs: usize) -> usize
    where Iter1: Iterator<Item = i32>,
          Iter2: Iterator<Item = i32>
{
    seq1.zip(seq2).take(nb_pairs).filter(|&p| p.0 % MASK == p.1 % MASK).count()
}

fn solve((factor1, factor2): (i32,i32), nb_pairs: usize, with_criteria: bool) -> usize {
    let seq1 = new_seq(16807, factor1);
    let seq2 = new_seq(48271, factor2);
    if with_criteria {
        compare_seqs(seq1.filter(|&n| n % 4 == 0),
                     seq2.filter(|&n| n % 8 == 0),
                     nb_pairs)
    } else {
        compare_seqs(seq1, seq2, nb_pairs)
    }
}

fn main() {
    let input = parse(&lib::read_input_file().unwrap()).unwrap();
    println!("{}", solve(input, 40_000_000, false));
    println!("{}", solve(input, 5_000_000, true));
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(solve((65, 8921), 5, false), 1)
    }
}
