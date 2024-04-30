use std::collections::HashMap;

pub type Vertex = String;
pub type AdjacencyList = HashMap<String,Vec<String>>;

#[derive(Debug)]
pub struct GameData {
    pub seconds: usize,
    pub hbfs_path: String,
    pub path_len: usize,
    pub start_link: String,
    pub end_link: String,
    pub rating: Option<usize>,
}

pub fn directed_adjacency(edges: Vec<(Vertex, Vertex)>) -> AdjacencyList {
    // init hashmap of linkname and vector of directed connections
    let mut graph_list: AdjacencyList = AdjacencyList::new();
    for (v, w) in edges {
        // if v exists, push w to end, else add new vec
        graph_list.entry(v).or_insert_with(Vec::new).push(w);
    }
    graph_list
}

pub fn h_graph_info(game_info: Vec<(String,String,String)>) -> Vec<GameData> {
    /*  function takes read data from tsv file
        builds GameData for each player run
        holds connected pairs in a vector a GameData
    */
    let mut game_data: Vec<GameData> = Vec::new();

    for data in game_info {   
        let time: usize = data.0.parse().unwrap();
        let path: String = data.1.parse().unwrap();
        let path_parts: Vec<&str> = path.split(';').collect();
        let length: usize = path_parts.len();
        
        //handle if empty path
        let start = if let Some(part) = path_parts.get(0) {
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
            path_len: length,
            start_link: start,
            end_link: end,
            rating: grade
        };
        game_data.push(game_datum);
    }
    game_data
}

// pub fn h_graph_info(game_info: Vec<(String,String,String)>) -> Vec<GameData> {
//     /*  function takes read data from tsv file
//         builds GameData for each player run
//         holds connected pairs in a vector a GameData
//     */
//     let mut game_data: Vec<GameData> = Vec::new();
//     let mut start_link_count: HashMap<String, usize> = HashMap::new();

//     for data in game_info {   
//         let time: usize = data.0.parse().unwrap();
//         let path: String = data.1.parse().unwrap();
//         let path_parts: Vec<&str> = path.split(';').collect();
//         let length: usize = path_parts.len();
        
//         //handle if empty path
//         let start = if let Some(part) = path_parts.get(0) {
//             part.to_string()
//         } else {
//             "".to_string()
//         };

//         let end = if let Some(part) = path_parts.last() {
//             part.to_string()
//         } else {
//             "".to_string()
//         };

//         //handle if grade null or contains number 1-5
//         let grade = if data.2 == "Null" {
//             None
//         } else {
//             data.2.parse().ok()
//         };

//         // Increment the count of the start link
//         let count = start_link_count.entry(start.clone()).or_insert(0);
//         *count += 1;

//         // Only add the game data if the start link has been seen less than three times
//         if *count <= 3 {
//             let game_datum = GameData {
//                 seconds: time,
//                 hbfs_path: path,
//                 path_len: length,
//                 start_link: start,
//                 end_link: end,
//                 rating: grade
//             };
//             game_data.push(game_datum);
//         }
//     }
//     game_data
// }
