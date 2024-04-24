use std::collections::HashMap;

pub fn directed_adjacency(edges: Vec<(String, String)>) -> HashMap<String, Vec<String>> {
    // init hashmap of linkname and vector of directed connections
    let mut graph_list: HashMap<String, Vec<String>> = HashMap::new();
    for (v, w) in edges {
        // if v exists, push w to end, else add new vec
        graph_list.entry(v).or_insert_with(Vec::new).push(w);
    }
    graph_list
}