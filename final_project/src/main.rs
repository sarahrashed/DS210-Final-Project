#![allow(warnings)]
use urlencoding::decode;
mod reading; 
mod graph_structure;
use crate::graph_structure::AdjacencyList;
use crate::graph_structure::h_graph_info;
mod search;

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
    //println!("{:?}",adj_list["14th_century"][0]);

    let read_in_game = reading::read_game_connections("../../wikispeedia_paths-and-graph/paths_finished.tsv");

    let data = h_graph_info(read_in_game);
    println!("{:?}",data[0].start_link);

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
