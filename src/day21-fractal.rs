mod lib;

type Rules = std::collections::HashMap<Vec<bool>, Vec<bool>>;

fn run(rules: &Rules, nb_iter: usize) -> Vec<bool> {
    let mut state = vec![false, true, false,
                         false, false, true,
                         true, true, true];
    for _ in 0..nb_iter {
        let cur_side = (state.len() as f64).sqrt() as usize;
        let cur_cell_size = if cur_side % 2 == 0 {2} else {3};
        let cells_per_side = cur_side / cur_cell_size;
        let nxt_cell_size = cur_cell_size + 1;
        let nxt_side = cells_per_side * nxt_cell_size;
        let mut new_state = vec![false; nxt_side * nxt_side];
        for i in 0..cells_per_side {
            for j in 0..cells_per_side {
                let mut key = vec![false; cur_cell_size * cur_cell_size];
                for (k, row) in key.chunks_mut(cur_cell_size).enumerate() {
                    let ofs =
                        i * cur_side * cur_cell_size +
                        k * cur_side +
                        j * cur_cell_size;
                    row.copy_from_slice(&state[ofs .. ofs + cur_cell_size]);
                }
                let new_cell = rules.get(&key).expect("not found");
                for (k, row) in new_cell.chunks(nxt_cell_size).enumerate() {
                    let ofs =
                        i * nxt_side * nxt_cell_size +
                        k * nxt_side +
                        j * nxt_cell_size;
                    new_state[ofs .. ofs + nxt_cell_size].copy_from_slice(row)
                }
            }
        }
        state = new_state;
    }
    state
}

fn nb_values_alive(rules: &Rules, nb_iter: usize) -> usize {
    let state = run(rules, nb_iter);
    state.iter().filter(|&&v| v).count()
}

fn main() {
    let input = &lib::read_input_file().unwrap();
    let rules = parse_and_compile(input).unwrap();
    println!("{}", nb_values_alive(&rules, 5));
    println!("{}", nb_values_alive(&rules, 18));
}

fn parse_and_compile(input: &str) -> Option<Rules> {
    let mut rules : Rules = Default::default();
    for rule in input.trim().lines().map(parse_row) {
        match rule {
            None => return None,
            Some((side, cell0, cell2)) => {
                for &ti in [true,false].iter() {
                    for &tj in [true,false].iter() {
                        for r in 0..4 {
                            let mut cell0trans = vec![false; cell0.len()];
                            for (idx, &value) in cell0.iter().enumerate() {
                                let i = idx/side;
                                let j = idx%side;
                                let mut i = if ti {side-1-i} else {i};
                                let mut j = if tj {side-1-j} else {j};
                                for _ in 0..r {
                                    j = side-1-j;
                                    std::mem::swap(&mut i, &mut j)
                                }
                                cell0trans[i*side + j] = value;
                            }
                            rules.insert(cell0trans.clone(), cell2.clone());
                        }
                    }
                }
            }
        }
    }
    Some(rules)
}

fn parse_row(input: &str) -> Option<(usize, Vec<bool>, Vec<bool>)> {
    let mut toks = input.split(' ');
    let tok0 = toks.next()?;
    let _tok1 = toks.next(); // =>
    let tok2 = toks.next()?;
    let side = match tok0.len() {
        5 => Some(2),
        11 => Some(3),
        _ => None,
    }?;
    let new_side = side + 1;
    let parse_cell = |str: &str, side: usize| {
        let mut values = vec![false; side*side];
        for (i, row) in str.split("/").enumerate() {
            for (j, tok) in row.as_bytes().iter().cloned().enumerate() {
                values[i*side + j] = match tok {
                    b'#' => true,
                    b'.' => false,
                    _ => panic!(),
                }
            }
        }
        values
    };
    let cell0 = parse_cell(tok0, side);
    let cell2 = parse_cell(tok2, new_side);
    Some((side, cell0, cell2))
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let input = r#"
../.# => ##./#../...
.#./..#/### => #..#/..../..../#..#
"#;
        let rules = parse_and_compile(input).unwrap();
        assert_eq!(vec![
            true , true , false, true , true , false,
            true , false, false, true , false, false,
            false, false, false, false, false, false,
            true , true , false, true , true , false,
            true , false, false, true , false, false,
            false, false, false, false, false, false,
        ], run(&rules, 2));
        assert_eq!(12, nb_values_alive(&rules, 2));
    }

}
