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

// impl GameData {
//     fn build_data(seconds: usize, hbfs_path: String, start_link: String, end_link: String, rating: Option<usize>) -> GameData {
//         seconds,
//         hbfs_path,
//         start_link,
//         end_link,
//         rating,
//     }
// }

fn main() {
    // let encoded = "%C3%81ed%C3%A1n_mac_Gabr%C3%A1in";
    // let decoded = decode(encoded).unwrap();

    // println!("{}",decoded);

    let read_in_links = reading::read_link_connections("../../wikispeedia_paths-and-graph/links.tsv");
    let adj_list = graph_structure::directed_adjacency(read_in_links);

    let read_in_game = reading::read_game_connections("../../wikispeedia_paths-and-graph/paths_finished.tsv");


    // let mut items = read_in_game.split('\t').skip(2);
    //     if let (Some(seconds), Some(hbfs_path), Some(rating)) = (items.next(), items.next(), items.next()) {
    //         let path_parts: Vec<&str> = hbfs_path.split(';').collect();
    //         let start = path_parts[1].to_string();
    //         let end = path_parts[path_parts.len() - 1].to_string();
    //         let rating = if rating == "null" {
    //             None
    //         } else {
    //             Some(rating.parse().unwrap())
    //         };
    //         let game_datum = GameData {
    //             seconds: seconds.parse().unwrap(),
    //             hbfs_path: hbfs_path.to_string(),
    //             start,
    //             end,
    //             rating,
    //         };
    //         game_info.push(game_datum);
    //     }


    let mut game_data = Vec::new();

    for data in read_in_game {
        //let (time, path, grade) = data;
        let time: usize = data.0.parse().unwrap();
        let path = data.1;
        let path_parts: Vec<&str> = path.split(';').collect();
        let start = path_parts[1].to_string();
        let end = path_parts[path.len() - 1].to_string();
        
        //check if grade is null value or number 1-5
        let grade = if data.2 == "Null" {
            None
        } else {
            let grade: usize = data.2.parse().unwrap();
            Some(grade)
        };
    
        // below is crap fix rating so some/none depending on if null
        // let game_datum = GameData {
        //     seconds: time,
        //     hbfs_path: path,
        //     start_link: start,
        //     end_link: end,
        //     rating: grade
        // };
        // game_data.push(game_datum);
        game_data.push(GameData(seconds:time, hbfs_path:path, start_link:start, end_link:end, rating: grade));
    }
    println!("{:?}",game_data);

}
