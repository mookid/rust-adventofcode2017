mod lib;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Group {
    Nil,
    Cons(Item, std::rc::Rc<Group>),
}

struct IterGroup<'a> {
    next: Option<(&'a Item, &'a std::rc::Rc<Group>)>,
}

impl Group {
    fn mk_iter(&self) -> Option<(&Item, &std::rc::Rc<Group>)> {
        match self {
            &Group::Nil => None,
            &Group::Cons(ref item, ref grp) => Some((item, grp)),
        }
    }
    pub fn iter(&self) -> IterGroup {
        IterGroup {
            next: Group::mk_iter(&self),
        }
    }
}

impl<'a> Iterator for IterGroup<'a> {
    type Item = &'a Item;
    fn next(&mut self) -> Option<Self::Item> {
        match self.next {
            None => None,
            Some((item, grp)) => {
                use std::ops::Deref;
                self.next = Group::mk_iter(grp.deref());
                Some(item)
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Item {
    Garbage(i32),
    Group(std::rc::Rc<Group>),
}

type Tok = u8;

fn parse(str: &str) -> Option<Group> {
    let mut iter = str.as_bytes().into_iter().cloned();
    let tok = iter.next();
    group(tok, &mut iter)
}

fn group<S: Iterator<Item = Tok>>(tok: Option<Tok>, str: &mut S) -> Option<Group> {
    match tok {
        Some(b'{') => {
            let tok = str.next();
            group_end(tok, str)
        },
        _ => None,
    }
}

fn group_end<S: Iterator<Item = Tok>>(tok: Option<Tok>, str: &mut S) -> Option<Group> {
    match tok {
        Some(b'}') => Some(Group::Nil),
        Some(_) => {
            let tok = match tok {
                Some(b',') => str.next(),
                tok => tok,
            };
            item(tok, str).and_then(|it| {
                let tok = str.next();
                group_end(tok, str).map(|end| Group::Cons(it, std::rc::Rc::new(end)))
            })
        },
        _ => None,
    }
}

fn item<S: Iterator<Item = Tok>>(tok: Option<Tok>, str: &mut S) -> Option<Item> {
    match tok {
        Some(b'{') => group(tok, str).and_then(|grp| {
            Some(Item::Group(std::rc::Rc::new(grp)))
        }),
        Some(b'<') => garbage(tok, str).and_then(|count| {
            Some(Item::Garbage(count))
        }),
        _ => None,
    }
}

fn garbage<S: Iterator<Item = Tok>>(tok: Option<Tok>, str: &mut S) -> Option<(i32)> {
    match tok {
        Some(b'<') => garbage_content(str.next(), str),
        _ => None,
    }
}

fn garbage_content<S: Iterator<Item = Tok>>(tok: Option<Tok>, str: &mut S) -> Option<(i32)> {
    match tok {
        Some(b'>') => Some(0),
        Some(b'!') => {
            let _ = str.next();
            garbage_content(str.next(), str).map(|count| count)
        },
        Some(_) => garbage_content(str.next(), str).map(|count| count + 1),
        _ => None,
    }
}

fn score(ast: &Group) -> i32 {
    1 + score_aux(ast, 1)
}

fn count_garbage(ast: &Group) -> i32 {
    ast.iter()
        .filter_map(|item| match item {
            &Item::Group(ref group) => Some(count_garbage(group)),
            &Item::Garbage(count) => Some(count),
        })
        .sum()
}

fn score_aux(ast: &Group, level: i32) -> i32 {
    ast.iter()
        .filter_map(|item| match item {
            &Item::Group(ref group) => Some(level + 1 + score_aux(&group, level + 1)),
            &Item::Garbage(_) => None,
        })
        .sum()
}

fn main() {
    let input = lib::read_input_file().unwrap();
    let ast = parse(input.as_str()).unwrap();
    println!("{}", score(&ast));
    println!("{}", count_garbage(&ast));
}

#[cfg(test)]
mod tests {
    use super::*;
    fn mk_test(input: &str, expected_pp: &str, expected_score: i32) {
        let ast = parse(input);
        println!("input = {}", input);
        println!("ast = {:?}", ast);
        assert_eq!(format!("{:?}", ast), expected_pp);
        assert_eq!(score(&ast.unwrap()), expected_score);
    }
    #[test]
    fn test1() {
        mk_test("{}", "Some(Nil)", 1)
    }
    #[test]
    fn test2() {
        mk_test("{{{}}}", "Some(Cons(Group(Cons(Group(Nil), Nil)), Nil))", 6)
    }
    #[test]
    fn test3() {
        mk_test("{{},{}}", "Some(Cons(Group(Nil), Cons(Group(Nil), Nil)))", 5)
    }
    #[test]
    fn test4() {
        mk_test("{{{},{},{{}}}}", "Some(Cons(Group(Cons(Group(Nil), Cons(Group(Nil), Cons(Group(Cons(Group(Nil), Nil)), Nil)))), Nil))", 16)
    }
    #[test]
    fn test5() {
        mk_test("{<a>,<a>,<a>,<a>}", "Some(Cons(Garbage(1), Cons(Garbage(1), Cons(Garbage(1), Cons(Garbage(1), Nil)))))", 1)
    }
    #[test]
    fn test6() {
        mk_test("{{<ab>},{<ab>},{<ab>},{<ab>}}", "Some(Cons(Group(Cons(Garbage(2), Nil)), Cons(Group(Cons(Garbage(2), Nil)), Cons(Group(Cons(Garbage(2), Nil)), Cons(Group(Cons(Garbage(2), Nil)), Nil)))))", 9)
    }
    #[test]
    fn test7() {
        mk_test("{{<!!>},{<!!>},{<!!>},{<!!>}}", "Some(Cons(Group(Cons(Garbage(0), Nil)), Cons(Group(Cons(Garbage(0), Nil)), Cons(Group(Cons(Garbage(0), Nil)), Cons(Group(Cons(Garbage(0), Nil)), Nil)))))", 9)
    }
    #[test]
    fn test8() {
        mk_test("{{<a!>},{<a!>},{<a!>},{<ab>}}", "Some(Cons(Group(Cons(Garbage(17), Nil)), Nil))", 3)
    }

    fn mk_garbage_test(input: &str, expected_count: i32) {
        let input = "{".to_owned() + input + "}";
        let input = input.as_str();
        let ast = parse(input);
        println!("input = {}", input);
        println!("ast = {:?}", ast);
        assert_eq!(count_garbage(&ast.unwrap()), expected_count);
    }

    #[test]
    fn test9() {
        mk_garbage_test("<>", 0)
    }
    #[test]
    fn test10() {
        mk_garbage_test("<random characters>", 17)
    }
    #[test]
    fn test11() {
        mk_garbage_test("<<<<>", 3)
    }
    #[test]
    fn test12() {
        mk_garbage_test("<{!>}>", 2)
    }
    #[test]
    fn test13() {
        mk_garbage_test("<!!>", 0)
    }
    #[test]
    fn test14() {
        mk_garbage_test("<!!!>>", 0)
    }
    #[test]
    fn test15() {
        mk_garbage_test(r#"<{o"i!a,<{i<a>"#, 10)
    }
}
