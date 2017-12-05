mod lib;

fn main() {
    let input : Vec<_> = lib::read_input_file().unwrap().trim()
        .lines().map(lib::parse_i32)
        .collect();
    println!("{}", solve(&input, false));
    println!("{}", solve(&input, true));
}

fn solve(ns : &Vec<i32>, complex_jump: bool) -> i32 {
    let mut ns = ns.clone();
    let mut i : i32 = 0;
    let len = ns.len() as i32;
    let mut counter = 0;
    while 0 <= i && i < len {
        let slot = &mut ns[i as usize];
        let delta = *slot;
        *slot += if complex_jump && 3 <= *slot {
            -1
        } else {
            1
        };
        i += delta;
        counter += 1;
    }
    counter
}
