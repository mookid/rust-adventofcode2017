mod lib;

fn solve(str: &[u8], halfway: bool) -> i32 {
    let len = str.len();
    let step = if halfway {
        len / 2
    } else {
        1
    };
    let mut sum : i32 = 0;
    for i in 0..len {
        let current = str[i];
        let next = str[(i + step) % len];
        if current == next {
            sum += (current - ('0' as u8)) as i32
        }
    }
    sum
}

fn main () {
    let input = lib::read_input_file().unwrap();
    let input = input.trim();
    println!("{}", solve(input.as_bytes(), false));
    println!("{}", solve(input.as_bytes(), true));
}
