#![allow(warnings)]
use urlencoding::decode;
mod reading; 
mod graph_structure;
use crate::graph_structure::AdjacencyList;
use crate::graph_structure::h_graph_info;
mod search;

fn main() {
    // let encoded = "%C3%81ed%C3%A1n_mac_Gabr%C3%A1in";
    // let decoded = decode(encoded).unwrap();

    // println!("{}",decoded);

    // get link connections
    let read_in_links = reading::read_link_connections("../../wikispeedia_paths-and-graph/links.tsv");

    // directed adjacency list from above connections
    let adj_list = graph_structure::directed_adjacency(read_in_links);

    // get game info: time, path, rating
    let read_in_game = reading::read_game_connections("../../wikispeedia_paths-and-graph/paths_finished.tsv");

    // build vector of GameData struct for easy access
    let data = h_graph_info(read_in_game);
    println!("{:?}",data.len());

    // loop through games
    //51318
    let mut bfs_player_path: Vec<usize> = vec![0;10000];
    for game_index in 0..10000 {
        let length = search::wiki_BFS(
            &adj_list, 
            &data[game_index].start_link, 
            &data[game_index].end_link
        );
        bfs_player_path[game_index] = length;
    }

    // println!("{:?}",bfs_player_path)
    // println!("{:?}",game_data[0].hbfs_path);
    // println!("{:?}",adj_list["14th_century"]);
    // println!("{:?}",adj_list["15th_century"]);
    // println!("{:?}",search::wiki_BFS(&adj_list, &game_data[0].start_link, &game_data[0].end_link));


    // USE BELOW AS A TEST!!!!!
    // let mut graph_list = AdjacencyList::new();
    // graph_list.insert("A".to_string(), vec!["B".to_string(), "C".to_string()]);
    // graph_list.insert("B".to_string(), vec!["C".to_string(), "D".to_string()]);
    // graph_list.insert("C".to_string(), vec!["D".to_string()]);
    // graph_list.insert("D".to_string(), vec!["A".to_string()]);

    // let start = "A";
    // let stop = "D"; 
    // println!("{:?}",graph_list);
    // println!("{:?}",search::wiki_BFS(&graph_list, &start, &stop));
}
