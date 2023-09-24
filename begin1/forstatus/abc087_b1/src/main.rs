use proconio::input;

fn main() {
    input!{
        a:i64,
        b:i64,
        c:i64,
        x:i64,
    }
    let mut ans:i64=0;
    for j in 0..=a{
        for k in 0..=b{
            for m in 0..=c{
                let now_sum:i64=j*500+k*100+m*50;
                if now_sum == x{
                    ans=ans+1;
                }
            }
        }
    }
    println!("{}",ans);
}
