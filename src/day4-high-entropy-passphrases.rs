mod lib;

fn solve(input: &str, filter_anagrams: bool) -> usize {
    input.lines().filter(|line| {
        let mut toks :Vec<_> = line.split(char::is_whitespace)
            .map(String::from)
            .collect();
        let count_all = toks.len();
        if filter_anagrams {
            toks.iter_mut().for_each(|tok| {
                use std::iter::FromIterator;
                let mut chars : Vec<_> = tok.chars().collect();
                chars.sort();
                std::mem::replace(tok, String::from_iter(chars));
            })
        };
        toks.sort();
        toks.dedup();
        let count_unique = toks.len();
        count_all == count_unique
    })
        .count()
}

fn main() {
    let input = lib::read_input_file().unwrap();
    let input = input.trim();
    println!("{}",solve(input, false));
    println!("{}",solve(input, true));
}
