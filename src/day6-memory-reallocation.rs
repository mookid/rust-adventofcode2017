mod lib;

fn normalize(blocks : &mut Vec<i32>) -> Option<()> {
    let nb_blocks = blocks.len();
    let idx_max = (0..nb_blocks).rev().max_by_key(|&i| blocks[i])?;
    let k = std::mem::replace(&mut blocks[idx_max], 0) as usize;
    for i in idx_max + 1 .. idx_max + k + 1 {
        blocks[i % nb_blocks] += 1;
    }
    Some(())
}

fn solve(blocks : &Vec<i32>, loop_size: bool) -> Option<i32> {
    let mut blocks = blocks.clone();
    let mut tbl = std::collections::HashMap::new();
    for i in 0.. {
        match tbl.insert(blocks.clone(), i) {
            Some(first_encounter) =>
                return Some(if loop_size {
                    i - first_encounter
                } else {
                    i
                }),
            None => (),
        }
        normalize(&mut blocks);
    }
    panic!()
}

fn main() {
    let input = lib::read_input_file().unwrap();
    let input = input.trim()
        .split(char::is_whitespace)
        .map(lib::parse_i32)
        .collect::<Vec<_>>();
    println!("{}", solve(&input, false).unwrap());
    println!("{}", solve(&input, true).unwrap());
}
