#![allow(warnings)]
use urlencoding::decode;
mod reading; 
mod graph_structure;
use crate::graph_structure::AdjacencyList;
use crate::graph_structure::h_graph_info;
mod search;
use rand::seq::IteratorRandom;
use rand::{SeedableRng, thread_rng};
use rand::rngs::StdRng;

fn main() {
    // get link connections
    let read_in_links = reading::read_link_connections("../../wikispeedia_paths-and-graph/links.tsv");

    // directed adjacency list from above connections
    let adj_list = graph_structure::directed_adjacency(read_in_links);

    // get game info: time, path, rating
    let read_in_game = reading::read_game_connections("../../wikispeedia_paths-and-graph/paths_finished.tsv");

    // build vector of GameData struct for easy access
    let data = h_graph_info(read_in_game);
    //println!("{:?}",data.len());

    //take random sample from data
    let seed: [u8; 32] = [1; 32]; 
    let mut rng = StdRng::from_seed(seed);
    let sample_size: usize = 10_000;
    let sample = data.iter().choose_multiple(&mut rng, sample_size);

    // calculate true bfs path for game start and end vectors
    // start and ends come from sample of 10_000
    let mut bfs_player_path: Vec<usize> = vec![0;sample.len()];
    for game_index in 0..sample.len() {
        let length = search::wiki_BFS(
            &adj_list, 
            &sample[game_index].start_link, 
            &sample[game_index].end_link
        );
        bfs_player_path[game_index] = length;
    }

    // difference between best path and human path
    let mut error: Vec<usize> = vec![0;sample_size];

    // get metric sums
    let mut sum_bfs: usize = 0;
    let mut sum_hbfs: usize = 0;
    let mut sum_error: usize = 0;
    let mut max_index: usize = 0;

    for i in 0..sample_size {
        error[i] = sample[i].path_len - bfs_player_path[i];
        sum_bfs += bfs_player_path[i]; 
        sum_hbfs += sample[i].path_len;
        sum_error += error[i];
        if error[i] > max_index {max_index = i}
    }

    //metrics: avg distance real bfs, avg distance hbfs, MAE
    let avg_bfs = sum_bfs as f64 / sample_size as f64;
    let avg_hbfs = sum_hbfs as f64 / sample_size as f64;
    let bfs_MAE = sum_error as f64 / sample_size as f64;    

    println!("-------------------------------FIRST 10 PATHS IN 10,000 SAMPLE-------------------------------");
    println!(
        "Start Vertex{}End Vertex{}BFS Error{}",
        " ".repeat(34),
        " ".repeat(29),
        " ".repeat(9)
    );

    for i in 0..10{
        let start = decode(&sample[i].start_link).unwrap();
        let end = decode(&sample[i].end_link).unwrap();
        let dot1 = ".".repeat(45 - start.len());
        let dot2 = ".".repeat(45 - end.len());
        println!("{:<width$}{} {:<width$}{} {:<2}", start, dot1, end, dot2, error[i], width = 0);
    }

    println!("Average Traverse Length: {}",avg_bfs);
    println!("Average Human Traverse Length: {}",avg_hbfs);
    println!("MAE of BFS versus HBFS Paths: {}",bfs_MAE);
    println!("BFS Path with Largest Error: {} --> {} (error = {})",sample[max_index].start_link, sample[max_index].end_link, error[max_index]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bfs_functionality() {
    // testing bfs functionality
    let mut graph_list = AdjacencyList::new();
    graph_list.insert("A".to_string(), vec!["B".to_string(), "C".to_string()]);
    graph_list.insert("B".to_string(), vec!["C".to_string(), "D".to_string()]);
    graph_list.insert("C".to_string(), vec!["D".to_string()]);
    graph_list.insert("D".to_string(), vec!["A".to_string()]);

    let start1 = "A";
    let stop1 = "D"; 
    let start2 = "c";
    let stop2 = "B";
    println!("{:?}",graph_list);
    let traverse1 = search::wiki_BFS(&graph_list, &start1, &stop1);
    let traverse2 = search::wiki_BFS(&graph_list, &start2, &stop2);

    assert_eq!(traverse1, 2);
    assert_eq!(traverse2, 0);
    assert_eq!(graph_list["A"] == ["B","C"], true);
    }

    #[test]
    fn test_game_graph() {

    }
}
