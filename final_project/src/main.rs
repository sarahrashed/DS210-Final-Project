use urlencoding::decode;
mod reading; 
use std::any::type_name;

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

fn main() {
    let encoded = "%C3%81ed%C3%A1n_mac_Gabr%C3%A1in";
    let decoded = decode(encoded).unwrap();

    println!("{}",decoded);

    // TODO: change to relative path
    let read_in = reading::read_link_connections("C:/Programming for Data Science/DS210-Final-Project/wikispeedia_paths-and-graph/links.tsv");
    //println!("{:?}", read_in);

    
    let my_variable = vec![(String::from("hello"), String::from("world"))];
    println!("Type of read_in: {}", type_of(&read_in));

}
