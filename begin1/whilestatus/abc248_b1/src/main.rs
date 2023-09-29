use proconio::input;
fn main() {
    input!{
        mut a:i64,
        b:i64,
        k:i64,
    }
    let mut cnt:u32=0;
    while a < b{
        a = a * k;
        cnt = cnt+1;
    }
    println!("{}",cnt);
}
