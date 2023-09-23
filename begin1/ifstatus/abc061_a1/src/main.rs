use proconio::input;
fn main() {
    input!{
        first:i32,
        second:i32,
        third:i32,
    }
    let ans:bool;
    ans = if (first <= third) && (third <= second){
        true
    }else{
        false
    };
    if ans{
        println!("Yes")
    }else{
        println!("No")
    };
}
