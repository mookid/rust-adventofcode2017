mod lib;

fn is_delimiter(c : char) -> bool {
    match c {
        '-' => true, '>' => true, // arrow
        '(' => true, ')' => true, // parenthesis
        ',' => true,
        c => c.is_whitespace()
    }
}

struct Item<'a> {
    name: &'a str,
    weight: i32,
    subs: Vec<&'a str>,
}

fn parse(line: &str) -> Item {
    let mut iter = line.split(is_delimiter).filter(|str| 0 < str.len());
    let name = iter.next().unwrap();
    let snd = iter.next().unwrap();
    let subs: Vec<_> = iter.collect();
    Item {
        name,
        weight: lib::parse_i32(snd),
        subs,
    }
}

fn find_root<'a>(items : &'a Vec<Item>) -> &'a str {
    let all_hold_programs : std::collections::HashSet<&str> =
        items.iter().flat_map(|&Item{ref subs, ..}| subs).cloned().collect();
    let all_programs = items.iter().map(|&Item{ref name, ..}| name).cloned().collect();
    let roots = &all_programs - &all_hold_programs;
    roots.iter().nth(0).unwrap()
}

fn find_by_name<'a>(items : &'a Vec<Item>, x: &str) -> Option<&'a Item<'a>> {
    items.iter().find(|&&Item{name, ..}| name == x)
}

fn find_weight<'a>(tbl: &mut std::collections::HashMap<&'a str, i32>,
                   items : &'a Vec<Item>, name: &'a str) -> i32 {
    match tbl.get(name) {
        Some(&score) => score,
        None => {
            let &Item{ref subs, ref weight, ..} = find_by_name(items, name).unwrap();
            let subs_sum : i32 = subs.iter().map(|name| find_weight(tbl, items, name)).sum();
            let tot_weight = weight + subs_sum;
            tbl.insert(name, tot_weight);
            tot_weight
        },
    }
}

struct T<'a> {
    name: &'a str,
    nb: i32,
    value: Option<i32>,
}

fn find_adjusted_weight(items : &Vec<Item>, root: &str) -> i32 {
    let mut tbl = std::collections::HashMap::new();
    let mut name = root;
    let mut diff = 0;
    let mut cur_weight;
    let new_t = |name, value| T {
        name,
        value,
        nb: 1,
    };
    fn incr(t: T) -> T {
        T { nb: t.nb + 1, .. t }
    };
    loop {
        let &Item{ref subs, ref weight, .. } = find_by_name(items, name).unwrap();
        let (T {name: name1, nb: nb1, value: value1,},
             T {name: name2, nb: _, value: value2,})
            = subs.iter()
            .map(|name| (name, find_weight(&mut tbl, items, name)))
            .fold((new_t("", None), new_t("", None)),
                  |(t1, t2), (name, v)| match (t1.value, t2.value, v) {
                      (None, None, v) => (new_t(name, Some(v)), t2),
                      (None, Some(_), _) => panic!(),
                      (Some(w1), _, v) if v == w1 => (incr(t1), t2),
                      (Some(_), None, v) => (t1, new_t(name, Some(v))),
                      (Some(_), Some(w2), v) if v == w2 => (t1, incr(t2)),
                      (Some(_), Some(_), _) => panic!(),
                  });
        cur_weight = weight;
        if let (Some(w1), Some(w2)) = (value1,value2) {
            if nb1 == 1 {
                name = name1;
                diff = w2 - w1
            } else {
                name = name2;
                diff = w1 - w2;
            };
        } else {
            break;
        };
    }
    cur_weight + diff
}

fn main() {
    let input = lib::read_input_file().unwrap();
    let input : Vec<_> = input.lines().map(parse).collect();
    let root = find_root(&input);
    println!("{}", root);
    println!("{}", find_adjusted_weight(&input, &root))
}
