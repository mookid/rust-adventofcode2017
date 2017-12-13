mod lib;

#[derive(Copy, Clone, Debug)]
enum Hex {
    N,
    NE,
    SE,
    S,
    SW,
    NW,
}

const HEX_TABLE : [(&'static str, Hex); 6] =
    [
        ("n", Hex::N),
        ("ne", Hex::NE),
        ("se", Hex::SE),
        ("s", Hex::S),
        ("sw", Hex::SW),
        ("nw", Hex::NW),
    ];

fn nsteps_from_xy(p: (i32, i32)) -> i32 {
    let (mut x, mut y) = p;
    let mut counter = 0;
    while x != 0 && y != 0 {
        if x < 0 {
            x = -x
        }
        if y < 0 {
            y = -y
        }
        counter += 1;
        if 3.0/4.0 <= ((y*y) as f64) / (x*x + y*y) as f64 {
            y -= 2
        } else {
            x -= 1;
            y -= 1;
        }
    }
    counter + x + y
}

fn dir_from_hex(hex: Hex) -> (i32, i32) {
    match hex {
        Hex::N => (0,2),
        Hex::NE => (1,1),
        Hex::SE => (1,-1),
        Hex::S => (0,-2),
        Hex::SW => (-1,-1),
        Hex::NW => (-1,1),
    }
}

fn nsteps<Iter: Iterator<Item = Hex>>(it: Iter) -> i32 {
    nsteps_from_xy(it.map(dir_from_hex)
                   .fold((0,0), |(accx, accy), (x,y)| (accx + x, accy + y)))
}

fn nsteps_accumulated<Iter: Iterator<Item = Hex>>(it: Iter) -> i32 {
    it.map(dir_from_hex)
        .scan((0,0), |acc, (x,y)| {
            let (accx,accy) = *acc;
            *acc = (accx + x, accy + y);
            Some(*acc)
        })
        .map(nsteps_from_xy)
        .max()
        .unwrap()
}

fn parse(input: &str) -> Vec<Hex> {
    input.trim().split(",").map(|s| lib::parse_from(&HEX_TABLE, s)).collect()
}

fn main() {
    let input = parse(&lib::read_input_file().unwrap());
    println!("{}", nsteps(input.iter().cloned()));
    println!("{}", nsteps_accumulated(input.iter().cloned()));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(3, nsteps(parse("ne,ne,ne").iter().cloned()));
        assert_eq!(0, nsteps(parse("ne,ne,sw,sw").iter().cloned()));
        assert_eq!(2, nsteps(parse("ne,ne,s,s").iter().cloned()));
        assert_eq!(3, nsteps(parse("se,sw,se,sw,sw").iter().cloned()));
    }
}
