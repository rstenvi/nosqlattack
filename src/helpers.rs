
use std::str::FromStr;

pub fn split_in_two(s: &str, search: &str) -> (String, String)  {
    let split: (&str, &str) = match s.find(search)    {
        Some(i) => s.split_at(i),
        None    => (s, ""),
    };

    let part1 = split.0.to_string();
    let part2 = split.1.to_string().replace(search, "");
    (part1, part2)
}

pub fn parse_panic<T: FromStr>(s: &str) -> T {
    let p: T = match s.parse::<T>() {
        Ok(o) => o,
        Err(_) => panic!("Unable to parse string to type: {}", s),
    };
    p
}

pub fn search_vector(vec: Vec<String>, s: String) -> bool   {
    for v in vec.iter() {
        match s.find(v) {
            Some(_) => return true,
            None    => {},
        }
    }
    false
}

