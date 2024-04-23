use urlencoding::decode;
mod reading; 
mod graph_structure;

fn main() {
    // let encoded = "%C3%81ed%C3%A1n_mac_Gabr%C3%A1in";
    // let decoded = decode(encoded).unwrap();

    // println!("{}",decoded);

    // TODO: change to relative path
    let read_in_links = reading::read_link_connections("../../wikispeedia_paths-and-graph/links.tsv");

    for i in 0..3 {
        println!("{:?}", decode(&read_in_links[i].0).unwrap());
    }
    for i in 0..3 {
        println!("{:?}", read_in_links[i]);
    }

    let read_in_game = reading::read_game_connections("../../wikispeedia_paths-and-graph/paths_finished.tsv");

    for i in 0..3 {
        println!("{:?}", read_in_game[i]);
    }

    let f = graph_structure::reverse_edges(&vec![("a","b"),("c","c"),("f","z"),("z","f")]);
    println!("{:?}",f);

}
