use proconio::input;

fn main() {
    input!{
        s : String,
        t : String
    }
    let x:String=format!("{}{}",t,s);
    println!("{}",x);
}
