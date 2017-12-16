mod lib;

#[derive(Debug, Copy, Clone)]
enum Cmp {
    Ge,
    Le,
    Eq,
    Gt,
    Lt,
    Ne,
}

#[derive(Debug, Copy, Clone)]
enum Op {
    Inc,
    Dec,
}

const CMP_TABLE : [(&'static str, Cmp); 6] =
    [
        (">=", Cmp::Ge),
        ("<=", Cmp::Le),
        ("==", Cmp::Eq),
        (">", Cmp::Gt),
        ("<", Cmp::Lt),
        ("!=", Cmp::Ne),
    ];

const OP_TABLE : [(&'static str, Op); 2] =
    [
        ("inc", Op::Inc),
        ("dec", Op::Dec),
    ];

type Item<'a> = (&'a str, Op, i32, (&'a str, Cmp, i32));

fn parse(input: &str) -> Option<Item> {
    let mut iter = input.split(char::is_whitespace);
    let tok0 = iter.next()?;
    let tok1 = iter.next()?;
    let tok2 = iter.next()?;
    let tok3 = iter.next()?;
    let tok4 = iter.next()?;
    let tok5 = iter.next()?;
    let tok6 = iter.next()?;
    let reg1 = tok0;
    let op = lib::parse_from(&OP_TABLE, tok1);
    let delta = lib::parse_i32(tok2);
    if tok3 == "if" {
        let reg2 = tok4;
        let cmp = lib::parse_from(&CMP_TABLE, tok5);
        let bound = lib::parse_i32(tok6);
        Some((reg1, op, delta, (reg2, cmp, bound)))
    } else {
        None
    }
}

fn largest_register_value(input: &Vec<Item>, safe: bool) -> Option<i32> {
    let mut max = 0;
    let mut env = std::collections::HashMap::new();
    input.iter()
        .for_each(|&(reg1, op, delta, (reg2, cmp, bound))|{
            let &value2 = env.get(reg2).unwrap_or(&0);
            let cond = match cmp {
                Cmp::Ge => value2 >= bound,
                Cmp::Le => value2 <= bound,
                Cmp::Gt => value2 > bound,
                Cmp::Lt => value2 < bound,
                Cmp::Eq => value2 == bound,
                Cmp::Ne => value2 != bound,
            };
            if cond {
                let value1 = env.entry(reg1).or_insert(0);
                match op {
                    Op::Inc => *value1 += delta,
                    Op::Dec => *value1 -= delta,
                }
                if safe && max < *value1 {
                    max = *value1
                }
            }
        });
    if safe {
        Some(max)
    } else {
        env.iter().map(|(_, value)| *value).max()
    }
}

fn main() {
    let input = lib::read_input_file().unwrap();
    let input : Option<Vec<_>> = input.as_str().trim().lines().map(parse).collect();
    let input = input.unwrap();
    println!("{}", largest_register_value(&input, false).unwrap());
    println!("{}", largest_register_value(&input, true).unwrap());
}
