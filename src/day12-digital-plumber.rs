mod lib;

fn parse(str: &str) -> Vec<Vec<usize>> {
    fn delimiter_or_comma(c: char) -> bool {
        match c {
            ',' => true,
            c => char::is_whitespace(c),
        }
    };
    str.trim().lines()
        .enumerate()
        .map(|(i, line)| {
            let mut iter = line.split(delimiter_or_comma)
                .filter(|&str| str != "");
            let fst = iter.next();
            let snd = iter.next();
            assert_eq!(lib::parse_i32(fst.unwrap()) as usize, i);
            assert_eq!(snd.unwrap(), "<->");
            iter.map(|str| lib::parse_i32(str) as usize).collect()
        })
        .collect()
}

fn connected_components(tbl : Vec<Vec<usize>>) -> Vec<i32> {
    let mut all : Vec<i32> = std::iter::repeat(-1).take(tbl.len()).collect();
    for i in 0 .. tbl.len() {
        if all[i] == -1 {
            all[i] = i as i32;
            let mut todo = tbl[i].clone();
            while let Some(node) = todo.pop() {
                if all[node] == -1 {
                    let mut neighbours = tbl[node].clone();
                    todo.append(&mut neighbours);
                    all[node] = i as i32;
                }
            }
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
    let tbl = parse(&lib::read_input_file().unwrap());
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
"#);
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
