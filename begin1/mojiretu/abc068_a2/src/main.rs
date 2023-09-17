use proconio::input;
// use text_io::read;
fn main() {
    input!{
        prob:String,
    }
    let ans:String=format!("ABC{}",prob);
    println!("{}",ans);
}
