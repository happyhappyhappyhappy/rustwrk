use proconio::input;

fn main() {
    input!{
        a:i64,
        op:char,
        b:i64,
    }
    // println!("{} {} {}",a,op,b);
    let  ans = if op == '+'{
        a+b
    }else{
        a-b
    };
    println!("{}",ans);
}
