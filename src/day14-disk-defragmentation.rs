#![feature(splice)]
mod lib;

const INPUT : &'static str = "ugkiagan";
const GRID_SIZE : usize = 128;

fn bits_of_hexa(hex: u8) -> &'static [u8] {
    match hex {
        b'0' => b"0000", b'1' => b"0001", b'2' => b"0010", b'3' => b"0011",
        b'4' => b"0100", b'5' => b"0101", b'6' => b"0110", b'7' => b"0111",
        b'8' => b"1000", b'9' => b"1001", b'a' => b"1010", b'b' => b"1011",
        b'c' => b"1100", b'd' => b"1101", b'e' => b"1110", b'f' => b"1111",
        _ => panic!("bits_of_hexa")
    }
}

fn hex_to_bin(hex: &[u8]) -> Vec<u8> {
    let mut bin = vec!(b'0'; 4*hex.len());
    bin.chunks_mut(4).zip(hex.iter().cloned())
        .for_each(|(bin, hex)| bin.clone_from_slice(bits_of_hexa(hex)));
    bin
}

fn grid(input: &str) -> [u8; GRID_SIZE*GRID_SIZE] {
    let mut grid = [b'0'; GRID_SIZE*GRID_SIZE];
    grid.chunks_mut(GRID_SIZE).enumerate().for_each(|(i, row)| {
        row.clone_from_slice(&hex_to_bin(&lib::knot_hash(
            (input.to_owned() + "-" + &(i.to_string())).as_bytes())))
    });
    grid
}

fn solve(input: &str) -> usize {
    grid(input).iter().cloned().filter(|&bit| bit == b'1').count()
}

fn to_2d(x: usize) -> Option<(usize,usize)> {
    if x < GRID_SIZE*GRID_SIZE {
        Some((x/GRID_SIZE, x%GRID_SIZE))
    } else {
        None
    }
}

fn of_2d((i,j): (i32, i32)) -> Option<usize> {
    if 0 <= i && i < GRID_SIZE as i32 && 0 <= j && j < GRID_SIZE as i32 {
        Some(i as usize*GRID_SIZE +j as usize)
    } else {
        None
    }
}

fn solve2(input: &str) -> usize {
    let grid = grid(input);
    fn neighbours(x: usize) -> Vec<usize> {
        to_2d(x).map(|(i,j)| {
            let i = i as i32;
            let j = j as i32;
            [(i,j+1),(i,j-1),(i+1,j),(i-1,j)].iter().cloned()
                .filter_map(of_2d)
                .collect()
        })
            .unwrap_or(vec![])
    };

    let mut visited = [false; GRID_SIZE*GRID_SIZE];
    (0..grid.len()).fold(0,|acc, x| {
        if !visited[x] && grid[x] == b'1' {
            let mut todo = vec![x];
            while let Some(y) = todo.pop() {
                if to_2d(y).is_some() {
                    if !visited[y] && grid[y] == b'1' {
                        todo.extend_from_slice(&neighbours(y));
                        visited[y] = true;
                    }
                }
            }
            acc + 1
        } else {
            acc
        }
    })
}

fn main() {
    println!("{}", solve(INPUT));
    println!("{}", solve2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        let x = hex_to_bin(&lib::knot_hash("flqrgnkx-0".as_bytes()));
        let x : Vec<_> = x.iter().cloned().take(8).collect();
        assert_eq!(&x, &[b'1', b'1', b'0', b'1', b'0', b'1', b'0', b'0'])
    }

    #[test]
    fn test2() {
        assert_eq!(solve("flqrgnkx"), 8108)
    }
    #[test]
    fn test3() {
        assert_eq!(solve2("flqrgnkx"), 1242)
    }
}
