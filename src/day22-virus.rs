mod lib;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Status {
    Clean,
    Weakened,
    Infected,
    Flagged,
}

type XY = (i32,i32);
type Map = std::collections::HashMap<XY, Status>;

fn main() {
    let input = &lib::read_input_file().unwrap();
    let map = parse(&input);
    println!("{}", nb_infections(map.clone(), 10000, false));
    println!("{}", nb_infections(map, 10000000, true));
}

fn left(dir: Direction) -> Direction {
    match dir {
        Direction::Up =>
            Direction::Left,
        Direction::Left =>
            Direction::Down,
        Direction::Down =>
            Direction::Right,
        Direction::Right =>
            Direction::Up,
    }
}

fn right(dir: Direction) -> Direction {
    left(left(left(dir)))
}

fn reverse(dir: Direction) -> Direction {
    left(left(dir))
}

fn forward((x,y): XY, dir: Direction) -> XY {
    match dir {
        Direction::Up =>
            (x,y+1),
        Direction::Left =>
            (x-1,y),
        Direction::Down =>
            (x,y-1),
        Direction::Right =>
            (x+1,y),
    }
}

fn parse(input: &str) -> Map {
    let mut v = vec![];
    let mut nb_lines = 0;
    for (i,line) in input.lines().enumerate() {
        for (j, ch) in line.as_bytes().iter().enumerate() {
            match ch {
                &b'.' => (),
                &b'#' => v.push((i as i32, j as i32)),
                &c => panic!("unexpected character: {}", c),
            }
        }
        nb_lines += 1;
    }
    assert_eq!(nb_lines % 2, 1);
    let center = nb_lines/2;
    v.iter().map(|&(i,j)| ((j-center, center-i), Status::Infected))
        .collect()
}

fn nb_infections(map: Map, nb_steps: usize, v2: bool) -> usize {
    let (_, _, nb_infections, _) = run(map, nb_steps, v2);
    nb_infections
}

fn run(mut map: Map, nb_steps: usize, v2: bool) -> ((i32, i32), Direction, usize, Map) {
    let mut xy = (0,0);
    let mut dir = Direction::Up;
    let mut nb_infections = 0;
    for _ in 0..nb_steps {
        let mut entry = map.entry(xy).or_insert(Status::Clean);
        let (new_entry, new_dir) = match (v2, *entry) {
            (false, Status::Clean) => (Status::Infected, left(dir)),
            (false, Status::Infected) => (Status::Clean, right(dir)),
            (false, st) => panic!("illegal status: {:?}", st),
            (true, Status::Clean) => (Status::Weakened, left(dir)),
            (true, Status::Weakened) => (Status::Infected, dir),
            (true, Status::Infected) => (Status::Flagged, right(dir)),
            (true, Status::Flagged) => (Status::Clean, reverse(dir)),
        };
        *entry = new_entry;
        dir = new_dir;
        if *entry == Status::Infected {
            nb_infections += 1
        }
        xy = forward(xy, dir);
    }
    map.retain(|_, v| *v != Status::Clean);
    (xy, dir, nb_infections, map)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_nb_infections() {
        let mut map : Map = Default::default();
        map.insert((-1,0), Status::Infected);
        map.insert((1,1), Status::Infected);
        assert_eq!(5587, nb_infections(map, 10000, false))
    }

    #[test]
    fn test_map_state() {
        let mut map : Map = Default::default();
        map.insert((-1,0), Status::Infected);
        map.insert((1,1), Status::Infected);
        let (xy, dir, _, map) = run(map, 70, false);
        let elements : Map =
            [
                (-2,0), (-2,1),
                (-1,2),
                (0,0), (0,1), (0,3),
                (1,-1), (1,4),
                (2,-1), (2,4),
                (3,0), (3,3),
                (4,1), (4,2),
            ].iter().map(|xy| (*xy, Status::Infected)).collect();
        assert_eq!((1,1), xy);
        assert_eq!(dir, Direction::Up);
        assert_eq!(elements, map)
    }

    #[test]
    fn test_v2_100() {
        let mut map : Map = Default::default();
        map.insert((-1,0), Status::Infected);
        map.insert((1,1), Status::Infected);
        assert_eq!(26, nb_infections(map, 100, true))
    }

    #[test]
    fn test_parser() {
        let mut map : Map = Default::default();
        map.insert((-1,0), Status::Infected);
        map.insert((1,1), Status::Infected);
        let input = r#"
..#
#..
...
"#;
        assert_eq!(parse(input.trim()), map)
    }
}
