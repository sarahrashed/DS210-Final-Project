use urlencoding::decode;
mod reading; 
mod graph_structure;

#[derive(Debug)]
struct GameData {
    seconds: usize,
    hbfs_path: String,
    start_link: String,
    end_link: String,
    rating: Option<usize>,
}

fn main() {
    // let encoded = "%C3%81ed%C3%A1n_mac_Gabr%C3%A1in";
    // let decoded = decode(encoded).unwrap();

    // println!("{}",decoded);

    let read_in_links = reading::read_link_connections("../../wikispeedia_paths-and-graph/links.tsv");
    let adj_list = graph_structure::directed_adjacency(read_in_links);

    let read_in_game = reading::read_game_connections("../../wikispeedia_paths-and-graph/paths_finished.tsv");

    let mut game_data: Vec<GameData> = Vec::new();

    for data in read_in_game {   
        let time: usize = data.0.parse().unwrap();
        let path: String = data.1.parse().unwrap();
        let path_parts: Vec<&str> = path.split(';').collect();
        
        //handle if empty path
        let start = if let Some(part) = path_parts.get(1) {
            part.to_string()
        } else {
            "".to_string()
        };

        let end = if let Some(part) = path_parts.last() {
            part.to_string()
        } else {
            "".to_string()
        };

        //handle if grade null or contains number 1-5
        let grade = if data.2 == "Null" {
            None
        } else {
            data.2.parse().ok()
        };

        let game_datum = GameData {
            seconds: time,
            hbfs_path: path,
            start_link: start,
            end_link: end,
            rating: grade
        };
        game_data.push(game_datum);
    }
}
