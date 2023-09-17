use proconio::input;

fn main() {
    input!{
        a:u32,
        p:u32,
    }
    let ans:u32=(a*3+p)/2;
    println!("{}",ans);
}
