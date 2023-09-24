use proconio::input;
use proconio::marker::Chars;
fn main() {
    input!{
        strs:Chars,
    }
    let strlen:usize=strs.len();
    // TODO: 全検索 外れを引けば良いのか
    println!("{}",strlen);
}
