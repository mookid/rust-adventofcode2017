mod lib;

const INPUT : &'static str = "312051";

#[derive(Debug, Copy, Clone)]
enum CornerKind {
    UL, UR, DL, DR,
}

type Corner = (CornerKind,i32);

fn main() {
    let input = lib::parse_i32(INPUT);
    println!("{}", solve(input));
    println!("{}", solve2(input));
}

fn find_corner_after(n: i32) -> (Corner, i32) {
    for k in 0.. {
        for &kind in [CornerKind::UR, CornerKind::UL,
                      CornerKind::DL, CornerKind::DR].into_iter() {
            let corner :Corner = (kind, k);
            let sq = |x| x*x;
            let corner_value = match kind {
                CornerKind::UR => sq (2*k+1) - 6*k,
                CornerKind::UL => sq (2*k+1) - 4*k,
                CornerKind::DL => sq (2*k+1) - 2*k,
                CornerKind::DR => sq (2*k+1),
            };
            if n <= corner_value {
                return (corner, corner_value)
            }
        }
    }
    panic!()
}

fn manhattan_from_linear(n : i32) -> (i32, i32) {
    let (corner_after, corner_value) = find_corner_after(n);
    let (kind, k) = corner_after;
    // offset to the line center.
    // example: on line 17  16  15  14  13,
    // ofs(13) = -2
    // ofs(14) = -1
    // ofs(15) = 0
    // ofs(16) = 1
    // ofs(17) = 2
    let ofs = k - (corner_value - n);
    match kind {
        CornerKind::UR => (k, ofs),
        CornerKind::UL => (-ofs, k),
        CornerKind::DL => (-k, -ofs),
        CornerKind::DR => (ofs, -k),
    }
}

fn solve(n :i32) -> i32 {
    let (x,y) = manhattan_from_linear(n);
    x.abs() + y.abs()
}

fn solve2(n :i32) -> i32 {
    let mut tbl = std::collections::HashMap::new();
    tbl.insert((0,0), 1);
    for i in 2.. {
        let cur = manhattan_from_linear(i);
        let (x,y) = cur;
        let res = [(x-1, y-1), (x-1, y+0), (x-1, y+1),
                   (x+0, y-1), (x+0, y+1),
                   (x+1, y-1), (x+1, y+0), (x+1, y+1)]
            .into_iter()
            .filter_map(|p| tbl.get(p))
            .sum();
        n < res && return res;
        tbl.insert(cur, res);
    }
    panic!()
}
