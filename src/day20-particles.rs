mod lib;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Copy)]
struct V3 {
    x: i64,
    y: i64,
    z: i64,
}

impl std::ops::AddAssign for V3 {
    fn add_assign(&mut self, V3 {x, y, z}: Self) {
        self.x += x;
        self.y += y;
        self.z += z;
    }
}

impl V3 {
    fn abs(&self) -> i64 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

#[derive(Clone, Debug)]
struct Row {
    p: V3,
    v: V3,
    a: V3,
}

fn main() {
    let input = &lib::read_input_file().unwrap();
    let input = input.trim();
    let system = parse(input).unwrap();
    {
        let mut system = system.clone();
        run(&mut system, 1000, false);
        println!("{}", closest_particle(system));
    }
    {
        let mut system = system.clone();
        run(&mut system, 1000, true);
        println!("{}", nb_particles(system))
    }
}

type System = std::collections::HashMap<i32, Row>;

fn parse(str: &str) -> Option<System> {
    str.trim().lines().map(parse_row).enumerate()
        .map(|(i, res)| res.map(|res| (i as i32,res)))
        .collect()
}

fn parse_row(str: &str) -> Option<Row> {
    let mut it = str.split(',');
    let mut read_number = || {
        let mut neg = false;
        let mut res = 0;
        for c in it.next()?.as_bytes() {
            match c {
                &b'-' =>
                    neg = true,
                &c@b'0'...b'9' =>
                    res = res * 10 + c as i64 - b'0' as i64,
                _ =>
                    (),
            }
        }
        Some(if neg {-res} else {res})
    };
    let mut read_point = || {
        let x = read_number()?;
        let y = read_number()?;
        let z = read_number()?;
        Some(V3 {x, y, z})
    };
    let p = read_point()?;
    let v = read_point()?;
    let a = read_point()?;
    Some(Row {p, v, a})
}

fn closest_particle(system: System) -> i32 {
    let mut best_id = -1;
    let mut best_distance = 0;
    for i in system.keys() {
        let distance = system[i].p.abs();
        if distance < best_distance || best_id < 0 {
            best_distance = distance;
            best_id = *i;
        }
    }
    best_id
}

fn nb_particles(system: System) -> usize {
    system.len()
}

fn run(system: &mut System, nb_steps: usize, with_pruning: bool) {
    let mut position_map = std::collections::HashMap::new();
    for _ in 0..nb_steps {
        system.iter_mut().for_each(|(&i,x)| {
            x.v += x.a;
            x.p += x.v;
            if with_pruning {
                position_map.entry(x.p).or_insert_with(|| vec![]).push(i);
            }
        });
        for (_, v) in position_map.drain().filter(|&(_, ref v)| 1 < v.len()) {
            for i in v {
                system.remove(&i);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test1() {
        let input = r#"
p=< 3,0,0>, v=< 2,0,0>, a=<-1,0,0>
p=< 4,0,0>, v=< 0,0,0>, a=<-2,0,0>
"#;
        let input = input.trim();
        let mut system = parse(input).unwrap();
        run(&mut system, 5, false);
        assert_eq!(0, closest_particle(system));
    }


    #[test]
    fn test2() {
        let input = r#"
p=<-6,0,0>, v=< 3,0,0>, a=< 0,0,0>
p=<-4,0,0>, v=< 2,0,0>, a=< 0,0,0>
p=<-2,0,0>, v=< 1,0,0>, a=< 0,0,0>
p=< 3,0,0>, v=<-1,0,0>, a=< 0,0,0>
"#;
        let input = input.trim();
        let mut system = parse(input).unwrap();
        run(&mut system, 5, true);
        assert_eq!(1, nb_particles(system));
    }
}
