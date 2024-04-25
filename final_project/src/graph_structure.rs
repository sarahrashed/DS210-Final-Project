use std::collections::HashMap;

type Vertex = String;
type AdjacencyList = HashMap<String,Vec<String>>;

// #[derive(Debug)]
// struct Graph {
//     n: usize, // vertex labels in {0,...,n-1}
//     outedges: AdjacencyList,
// }

pub fn directed_adjacency(edges: Vec<(Vertex, Vertex)>) -> AdjacencyList {
    // init hashmap of linkname and vector of directed connections
    let mut graph_list: AdjacencyList = AdjacencyList::new();
    for (v, w) in edges {
        // if v exists, push w to end, else add new vec
        graph_list.entry(v).or_insert_with(Vec::new).push(w);
    }
    graph_list
}