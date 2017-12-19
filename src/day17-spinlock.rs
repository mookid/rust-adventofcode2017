mod lib;

type Container = (Vec<usize>, usize);
fn insert((mut vec, pos): Container, element: usize) -> Container {
    if pos + 1 == vec.len() {
        vec.push(element)
    } else {
        vec.insert(pos + 1, element)
    }
    (vec, pos + 1)
}

fn advance((vec, pos): Container, ofs: usize) -> Container {
    let len = vec.len();
    (vec, (pos + ofs) % len)
}

fn new() -> Container {
    (vec![0],0)
}

fn find_after((vec, _): Container, target: usize) -> Option<usize> {
    let len = vec.len();
    let i = vec.iter().position(|&x| x == target)?;
    Some(vec[(i+1) % len])
}

fn solve(nsteps: usize) -> Option<usize> {
    let nb_reps = 2017;
    let target = 2017;
    find_after((1..nb_reps+1).fold(new(), |st, i| insert(advance(st, nsteps), i)), target)
}

fn solve2(nsteps: usize) -> usize {
    (1..50_000_000).fold((0, 0), |(cell_value, cur_idx), i| {
        match ((cur_idx + nsteps) % i + 1) % (i+1) {
            1 => (i, 1),
            cur_idx => (cell_value, cur_idx),
        }
    }).0
}

fn main() {
    let input = lib::read_input_file().unwrap()
        .lines().next().unwrap()
        .parse::<usize>().unwrap();
    println!("{}", solve(input).unwrap());
    println!("{}", solve2(input));
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(solve(3), Some(638));
    }
}
