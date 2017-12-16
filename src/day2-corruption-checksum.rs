mod lib;

fn min_max<Iter, T>(mut numbers : Iter) -> std::option::Option<(T,T)>
    where Iter : Iterator<Item = T>,
          T: std::cmp::Ord + std::marker::Copy
{
    numbers.next().map(|first| {
        numbers.fold(
            (first,first),
            |(accmin, accmax), cur|
            (std::cmp::min(accmin, cur),
             std::cmp::max(accmax, cur)))
    })
}

fn solve_line1<Iter>(numbers: Iter) -> Option<i32>
    where Iter : Iterator<Item = i32>,
{
    min_max(numbers).map(|(min,max)| max - min)
}

fn solve_line2(numbers: &Vec<i32>) -> Option<i32>
{
    let len = numbers.len();
    for i in 0..len {
        let x = numbers[i];
        for j in 0..len {
            if i != j {
                let y = numbers[j];
                if y != 0 && x % y == 0 {
                    return Some(x / y)
                }
            }
        }
    }
    None
}

fn solve(input: &str, even: bool) -> Option<i32> {
    let mut iter = input.lines()
        .map(|line|
             line.trim()
             .split(char::is_whitespace)
             .map(lib::parse_i32))
        .map(|numbers|
             if even {
                 solve_line2(&numbers.collect())
             } else {
                 solve_line1(numbers)
             });
    let mut acc = 0;
    while let Some(x) = iter.next() {
        if let Some(x) = x {
            acc += x;
        } else {
            return None
        }
    }
    Some(acc)
}

fn main() {
    let input = lib::read_input_file().unwrap();
    println!("{}", solve(&input, false).unwrap());
    println!("{}", solve(&input, true).unwrap());
}
