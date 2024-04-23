use std::fs::File;
use std::io::{self, BufRead, BufReader};
use urlencoding::decode;

pub fn read_link_connections(path: &str) -> io::Result<Vec<(String, String)>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let actual_data: Vec<_> = reader
        .lines()
        .skip_while(|line| match line {
            Ok(line) => line.starts_with('#'),
            Err(_) => false,
        })
        .filter_map(|line| line.ok())
        .map(|line| {
            let mut pair = line.split('\t').map(String::from);
            (pair.next().unwrap_or_default(), pair.next().unwrap_or_default())
        })
        .collect();

    Ok(actual_data)
}


// pub fn read_link_connections(path: &str) -> Vec<(str,str)> {
//     //explore if better to init vec size or let grow when more space needed
//     let input = fs::read_to_string(path).unwrap()?;
//     let mut connections: Vec<(str,str)> = input
//         .lines();
//         .skip_while(|line| line.starts_with("#"));
//         .skip(1);
//         .map(|line| )
// }

// pub fn read_link_connections(path: &str) -> io::Result<Vec<(str,str)>> {
//     return Ok(BufReader::new(path).lines().map(|line| {
//         let line = line.unwrap();
//         let mut pair = line.split("\t").map(|s| s.parse::<str>().unwrap());
//         (pair.next().unwrap(), pair.next().unwrap())
//     }).collect::<Vec<(str,str)>())
// }


// //below is stack overflow solution
// use std::io::{self, prelude::*, BufReader};

// type Record = (u32, u32);

// fn read(content: &[u8]) -> io::Result<Vec<Record>> {
//     return Ok(BufReader::new(content).lines().map(|line| {
//         let line = line.unwrap();
//         let mut pair = line.split("\t").map(|s|s.parse::<u32>().unwrap());
//         (pair.next().unwrap(), pair.next().unwrap())
//     }).collect::<Vec<Record>>())
// }