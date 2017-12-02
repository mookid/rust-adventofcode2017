use ::std::io::Read;

pub fn read_input_file() -> Result<String, Box<::std::error::Error>> {
    let input_file = ::std::env::args().nth(1).ok_or("usage")?;
    let mut file = ::std::fs::File::open(input_file)?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;
    Ok(input)
}
