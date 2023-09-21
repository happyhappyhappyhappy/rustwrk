use proconio::input;

fn main() {
    input!{
        a:i64,
        b:i64,
    }
    let plus:i64=a+b;
    let minus:i64=a-b;
    let cross:i64=a*b;
    let mut ans:i64=std::i64::MIN;
    ans = if plus > ans {
        plus
    } else{
        ans
    };
    ans = if minus > ans {
        minus
    }else{
        ans
    };
    ans = if cross > ans{
        cross
    }else{
        ans
    };
    println!("{}",ans);
}
