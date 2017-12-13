#![allow(dead_code)]

use ::std::io::Read;

pub fn read_input_file() -> Result<String, Box<::std::error::Error>> {
    let input_file = ::std::env::args().nth(1).ok_or("usage")?;
    let mut file = ::std::fs::File::open(input_file)?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;
    Ok(input)
}

pub fn parse_i32(str: &str) -> i32 {
    str.parse::<i32>().unwrap()
}

pub fn parse_from<'a, T : Copy>(table: &'a [(&'static str, T)], input: &'a str) -> T {
    table.iter().find(|&&(key, _)| key == input)
        .map(|&(_, value)| value)
        .expect(("parse_from: ".to_owned() + &input.to_owned()).as_str())
}
