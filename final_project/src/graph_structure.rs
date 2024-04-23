type Vertex = &'static str;
type ListOfEdges = Vec<(Vertex,Vertex)>;
type AdjacencyLists = Vec<Vec<Vertex>>;

#[derive(Debug)]
pub struct Graph {
    outedges: AdjacencyLists,
}

// reverse direction of edges on a list
pub fn reverse_edges(list:&ListOfEdges) -> ListOfEdges {
    let mut new_list = vec![];
    for (u,v) in list {
        new_list.push((*v,*u));
    }
    new_list
}


impl Graph {
    pub fn add_directed_edges(&mut self, edges:&ListOfEdges) {
        for (u,v) in edges {
            self.outedges[*u].push(*v);
        }
    }
    // fn sort_graph_lists(&mut self) {
    //     for l in self.outedges.iter_mut() {
    //         l.sort();
    //     }
    // }
    pub fn create_directed(n:usize,edges:&ListOfEdges) -> Graph {
        let mut g = Graph{n,outedges:vec![vec![];n]};
        g.add_directed_edges(edges);
        //g.sort_graph_lists();
        g                                        
    }
    
    // fn create_undirected(n:usize,edges:&ListOfEdges)
    //                                         -> Graph {
    //     let mut g = Self::create_directed(n,edges);
    //     g.add_directed_edges(&reverse_edges(edges));
    //     g.sort_graph_lists();
    //     g                                        
    // }
}