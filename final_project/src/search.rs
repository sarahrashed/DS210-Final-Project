use std::collections::{HashMap, VecDeque, HashSet};
use crate::graph_structure::AdjacencyList;

pub fn wiki_BFS(adj_list: &AdjacencyList, start: &str, stop: &str) -> usize {
    // hold distance from start vertex
    let mut distance: HashMap<&str, usize> = HashMap::new();
    let mut queue = VecDeque::new();

    distance.insert(start, 0);
    queue.push_back(start);

    while let Some(curr_link) = queue.pop_front() {
        // break out of loop when hit stop vertex and return distance
        if curr_link == stop {
            return *distance.get(&curr_link).unwrap();
        }

        // if neighbors in adjacency list keep searching
        if let Some(neighbors) = adj_list.get(curr_link) {
            for neighbor in neighbors {
                if !distance.contains_key(neighbor.as_str()) {
                    distance.insert(neighbor.as_str(), distance[curr_link] + 1);
                    queue.push_back(neighbor);
                }
            }
        }
    }

    0
}

