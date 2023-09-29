use ascii::AsciiString;
fn main() {
    let sample = AsciiString::from_ascii("YesAscii".to_string()).unwrap();
    println!("{}",sample.as_str());
}
