use abbau_types::KsySchema;

fn main() {
    let arg = std::env::args().nth(1).unwrap();
    let text = std::fs::read_to_string(arg).unwrap();
    let schema: KsySchema = serde_yaml::from_str(&text).unwrap();
    println!("{:#?}", schema);
}
