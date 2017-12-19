mod lib;

fn solve(input: &str, filter_anagrams: bool) -> usize {
    input.lines().filter(|line| {
        let mut toks = line.split(char::is_whitespace)
            .map(String::from)
            .collect::<Vec<_>>();
        let count_all = toks.len();
        if filter_anagrams {
            toks.iter_mut().for_each(|tok| {
                let mut chars = tok.chars().collect::<Vec<_>>();
                chars.sort();
                std::mem::replace(tok, chars.iter().collect::<String>());
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
