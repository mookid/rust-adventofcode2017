mod lib;

fn main() {
    let input = &lib::read_input_file().unwrap();
    let input = parse(input);
    let lin = linearize(&input);
    println!("{}", letters(&lin));
    println!("{}", nb_steps(&lin));
}

fn parse(input: &str) -> Vec<Vec<u8>> {
    input.lines().filter(|str| str.len() != 0)
        .map(|str| str.as_bytes().iter().cloned().filter(|c| *c != b'`').collect())
        .collect()
}

fn non_empty(c: u8) -> bool {
    !c.is_ascii_whitespace()
}

#[derive(Clone, Copy, Debug)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

fn linearize(input: &Vec<Vec<u8>>) -> Vec<u8> {
    let mut res = vec![];
    if let Some(first_row) = input.iter().cloned().nth(0) {
        let nb_cols = first_row.len();
        let nb_rows = input.len();
        let ok = |y, x| {
            0 <= y && 0 <= x && {
                let y = y as usize;
                let x = x as usize;
                y < nb_rows && x < nb_cols && non_empty(input[y][x])
            }
        };
        if let Some(col) = first_row.iter().cloned().position(|c| non_empty(c)) {
            let mut y = 0;
            let mut x = col as i32;
            let mut dir = Dir::Down;
            while let Some(c) = input.iter().cloned().nth(y as usize)
                .and_then(|row| row.iter().cloned().nth(x as usize)) {
                    let nextdir = match c {
                        b' ' => break,
                        b'+' => match dir {
                            Dir::Left | Dir::Right => {
                                if ok(y+1, x) {
                                    Dir::Down
                                } else if ok(y-1, x) {
                                    Dir::Up
                                } else {
                                    break;
                                }
                            },
                            Dir::Up | Dir::Down => {
                                if ok(y, x+1) {
                                    Dir::Right
                                } else if ok(y, x-1) {
                                    Dir::Left
                                } else {
                                    break;
                                }
                            },
                        },
                        _ => dir,
                    };

                    match nextdir {
                        Dir::Up => y -= 1,
                        Dir::Down => y += 1,
                        Dir::Left => x -= 1,
                        Dir::Right => x += 1,
                    }
                    dir = nextdir;
                    res.push(c);
                }
        }
    }
    res
}

fn letters(input: &Vec<u8>) -> String {
    input.iter().cloned().filter(|c| c.is_ascii_alphabetic()).map(|c| c as char).collect()
}

fn nb_steps(input: &Vec<u8>) -> usize {
    input.iter().cloned().len()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let input = r#"
     |         `
     |  +--+   `
     A  |  C   `
 F---|----E|--+`
     |  |  |  D`
     +B-+  +--+`
"#;
        println!("{}", input);
        let input = parse(input);
        let lin = linearize(&input);
        assert_eq!("ABCDEF", letters(&lin));
        assert_eq!(38, nb_steps(&lin));
    }
}
