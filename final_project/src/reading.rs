use std::fs;

pub fn read_link_connections(path: &str) -> Vec<(String, String)> {
    /*  function reads link connections in tsv file
        skips over documentation marked by '#'
        holds connected pairs in a vector a Strings
    */
    let contents = fs::read_to_string(path)
        .expect("Cannot read the file!");

    let mut game_info = Vec::new();
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
            game_info.push((first, second));
        }
    }

    game_info
}

pub fn read_game_connections(path: &str) -> Vec<(String,String,String)> {
    /*  function reads game paths from tsv file
        skips over documentation marked by '#'
        holds info from game:
        seconds = time in seconds game played for path
        hbfs_path = human breadth first search path
        rating = hardness grade given by player, 1-5 or "Null" if skipped
    */
    let contents = fs::read_to_string(path)
        .expect("Cannot read the file!");

    let mut game_info = Vec::new();
    let mut skip_documentation = true;

    for line in contents.lines() {
        if skip_documentation {
            // Skip lines starting with "#"
            if !line.trim().starts_with('#') {
                skip_documentation = false;
            }
            continue;
        }
        
        // skip first two features in data
        let mut items = line.split('\t').skip(2);
        if let (Some(seconds), Some(hbfs_path), Some(rating)) = (items.next(), items.next(), items.next()) {
            game_info.push((seconds.parse().unwrap(), String::from(hbfs_path), String::from(rating)));
        }
    }

    game_info
}