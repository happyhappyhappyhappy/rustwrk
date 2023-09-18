use proconio::input;

fn main() {
    input!{
        x : char,
    }
    match x{
        'a' | 'i' | 'u' | 'e' | 'o' => println!("vowel"),
        _ => println!("consonant"),
    }
}
