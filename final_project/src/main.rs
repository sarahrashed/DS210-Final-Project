use urlencoding::decode;
mod reading; 

fn main() {
    let encoded = "%C3%81ed%C3%A1n_mac_Gabr%C3%A1in";
    let decoded = decode(encoded).unwrap();

    println!("{}",decoded);

    let read_in = reading::read_link_connections("C:/Programming for Data Science/DS210-Final-Project/wikispeedia_paths-and-graph/links.tsv");
    println!("{:?}", read_in);
}
