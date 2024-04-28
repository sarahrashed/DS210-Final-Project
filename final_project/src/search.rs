// use std::collections::VecDeque;
// mod graph_structure;

// pub fn BFS(adj_list: AdjacencyList, start: &str, stop: &str) -> Some(usize) {
//     //init hashmap for each link showing distance from start
//     let mut distance: HashMap<&str, usize> = HashMap::new();
//     let mut queue: VecDeque<Vertex> = VecDeque::new();

//     while queue[0] != stop {
//         let mut curr_link = queue.pop_front();
//         for link in graph.adj_list[curr_link].iter() {
//             if let None = distance[link] {
//                 distance[link] = Some(distance[curr_link].unwwrap() +1);
//                 queue.push_back(link);
//             }
//         }
//     }

//     distance[stop];
// }

use std::collections::{HashMap, VecDeque};
use crate::graph_structure::AdjacencyList;

pub fn wiki_BFS(adj_list: &AdjacencyList, start: &str, stop: &str) -> Option<usize> {
    let mut distance: HashMap<String, usize> = HashMap::new();
    let mut queue = VecDeque::new();

    distance.insert(start.to_string(), 0);
    queue.push_back(start.to_string());

    while let Some(curr_link) = queue.pop_front() {
        if curr_link == stop {
            return distance.get(&curr_link).copied();
        }

        if let Some(neighbors) = adj_list.get(&curr_link) {
            for neighbor in neighbors {
                if !distance.contains_key(neighbor) {
                    distance.insert(neighbor.clone(), distance[&curr_link] + 1);
                    queue.push_back(neighbor.clone());
                }
            }
        }
    }

    None
}
