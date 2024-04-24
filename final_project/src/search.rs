use std::collections::VecDeque;

pub fn BFS()
println!("{:?}",queue);
while let Some(v) = queue.pop_front() { // new unprocessed vertex
    println!("top {:?}",queue);
    for u in graph.outedges[v].iter() {
        // if let is a REPLACEMENT for a match statement
        // enum type so use pattern matching vs allowing clone, copy, debug, etc.
        if let None = distance[*u] { // consider all unprocessed neighbors of v
            distance[*u] = Some(distance[v].unwrap() + 1);
            queue.push_back(*u);
            println!("In {:?}",queue);
        }
    }
};