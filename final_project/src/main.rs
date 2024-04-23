use urlencoding::decode;

fn main() {
    let encoded = "%C3%81ed%C3%A1n_mac_Gabr%C3%A1in";
    let decoded = decode(encoded).unwrap();

    println!("{}",decoded);
}
