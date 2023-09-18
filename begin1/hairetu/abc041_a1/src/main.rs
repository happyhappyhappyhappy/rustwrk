use proconio::input;

fn main() {
    input!{
        s : proconio::marker::Chars,
        point : usize,
    }
    println!("{}",s[point-1]);
}
