mod lib;

fn parse(str: &str) -> Option<Vec<Vec<usize>>> {
    fn delimiter_or_comma(c: char) -> bool {
        match c {
            ',' => true,
            c => char::is_whitespace(c),
        }
    };
    fn parse_line((i, line): (usize, &str)) -> Option<Vec<usize>> {
        let mut iter = line.split(delimiter_or_comma)
            .filter(|&str| str != "");
        let fst = iter.next()?;
        let snd = iter.next()?;
        if lib::parse_i32(fst) as usize == i && snd == "<->" {
            iter.map(|str| lib::parse_i32(str))
                .map(|i32| if 0 <= i32 {
                    Some(i32 as usize)
                } else {
                    None
                })
                .collect()
        } else {
            None
        }
    };
    str.trim().lines()
        .enumerate()
        .map(parse_line)
        .collect()
}

fn connected_components(tbl : Vec<Vec<usize>>) -> Vec<i32> {
    let mut all = std::iter::repeat(-1).take(tbl.len()).collect::<Vec<_>>();
    for i in 0 .. tbl.len() {
        all[i] != -1 && continue;
        all[i] = i as i32;
        let mut todo = tbl[i].clone();
        while let Some(node) = todo.pop() {
            all[node] != -1 && continue;
            todo.extend_from_slice(&tbl[node]);
            all[node] = i as i32;
        }
    }
    all
}

fn component_size(connected_components: &Vec<i32>, n: usize) -> usize {
    connected_components.iter().cloned().filter(|&c| c == connected_components[n]).count()
}

fn count_components(connected_components: &Vec<i32>) -> usize {
    let mut cc = connected_components.clone();
    cc.sort();
    cc.dedup();
    cc.iter().count()
}

fn main() {
    let str = lib::read_input_file().unwrap();
    let tbl = parse(&str).unwrap();
    let components = connected_components(tbl);
    println!("{}", component_size(&components, 0));
    println!("{}", count_components(&components));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        let tbl = parse(r#"
0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5
"#).unwrap();
        println!("{:?}", tbl);
        assert_eq!(tbl, vec![
            vec![2],
            vec![1],
            vec![0, 3, 4],
            vec![2, 4],
            vec![2, 3, 6],
            vec![6],
            vec![4, 5],
        ]);

        let components = connected_components(tbl);
        assert_eq!(component_size(&components, 0), 6);
        assert_eq!(count_components(&components), 2);
    }
}
