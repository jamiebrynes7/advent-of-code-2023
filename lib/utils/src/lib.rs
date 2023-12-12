use std::str::FromStr;

pub mod grid;
pub mod nums;

pub fn parse_lines<T, E>(data: &str) -> Result<Vec<T>, E>
where
    T: FromStr<Err = E>,
{
    parse_lines_iter(data.split("\n"))
}

pub fn parse_lines_iter<'a, I, T, E>(iter: I) -> Result<Vec<T>, E>
where
    I: Iterator<Item = &'a str>,
    T: FromStr<Err = E>,
{
    iter.filter(|s| !s.is_empty())
        .map(|s| T::from_str(s))
        .collect()
}
