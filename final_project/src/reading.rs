use std::fs;

pub fn read_link_connections(path: &str) -> Vec<(String, String)> {
    //
    let contents = fs::read_to_string(path)
        .expect("Cannot read the file!");

    let mut pairs = Vec::new();
    //boolean flag
    let mut skip_documentation = true;

    for line in contents.lines() {
        if skip_documentation {
            // Skip lines starting with "#" aka documentation
            if !line.trim().starts_with('#') {
                skip_documentation = false;
            }
            continue;
        }
        
        let mut pair = line.split('\t').map(String::from);
        if let (Some(first), Some(second)) = (pair.next(), pair.next()) {
            pairs.push((first, second));
        }
    }

    pairs
}

pub fn read_game_connections(path: &str) -> Vec<(usize, String, String)> {
    let contents = fs::read_to_string(path)
        .expect("Cannot read the file!");

    let mut pairs = Vec::new();
    let mut skip_documentation = true;

    for line in contents.lines() {
        if skip_documentation {
            // Skip lines starting with "#"
            if !line.trim().starts_with('#') {
                skip_documentation = false;
            }
            continue;
        }
        
        let mut items = line.split('\t').skip(2);
        if let (Some(time), Some(travel_path), Some(difficulty)) = (items.next(), items.next(), items.next()) {
            pairs.push((time.parse().unwrap(), String::from(travel_path), String::from(difficulty)));
        }
    }

    pairs
}