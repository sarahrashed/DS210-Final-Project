use std::io::{self, Bufread};

pub fn read_links(path: &str) -> Vec<(str,str)> {
    //explore if better to init vec size or let grow when more space needed
    let input = fs::read_to_string(path).unwrap()?;
    let mut connections: Vec<(str,str)> = input
        .lines();
        .skip_while(|line| line.starts_with("#"));
        .skip(1);
        .map(|line| )
}