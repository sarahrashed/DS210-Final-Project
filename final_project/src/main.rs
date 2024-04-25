use urlencoding::decode;
mod reading; 
mod graph_structure;

fn main() {
    // let encoded = "%C3%81ed%C3%A1n_mac_Gabr%C3%A1in";
    // let decoded = decode(encoded).unwrap();

    // println!("{}",decoded);

    let read_in_links = reading::read_link_connections("../../wikispeedia_paths-and-graph/links.tsv");

    // for i in 0..3 {
    //     println!("{:?}", decode(&read_in_links[i].0).unwrap());
    // }
    // for i in 0..3 {
    //     println!("{:?}", read_in_links[i]);
    // }

    let read_in_game = reading::read_game_connections("../../wikispeedia_paths-and-graph/paths_finished.tsv");

    // for i in 0..3 {
    //     println!("{:?}", read_in_game[i]);
    // }

    let adj_list = graph_structure::directed_adjacency(read_in_links);

    //init hashmap of key = tuple pair of from->to, value = distance
    //loop through list of searches
    //call bfs on key pair and have bfs return distance to store in hashmap
    // for feature in read_in_game.iter().take(5) {
    //     let links: Vec<&str> = feature.1.split(";").collect();
    //     println!("{:?}", links);
    // }    

    for feature in read_in_game.iter().take(5) {
        println!("{:?}",feature);
    }
}
