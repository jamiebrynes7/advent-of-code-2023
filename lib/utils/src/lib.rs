use std::str::FromStr;

pub mod grid;

pub fn parse_lines<T, E>(data: &str) -> Result<Vec<T>, E>
where
    T: FromStr<Err = E>,
{
    data.split("\n")
        .filter(|s| !s.is_empty())
        .map(|s| T::from_str(s))
        .collect()
}
