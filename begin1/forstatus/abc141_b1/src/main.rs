use proconio::input;
use proconio::marker::Chars;
fn main() {
    input!{
        strs:Chars,
    }
    let strlen:usize=strs.len();
     let mut result:bool=true;
    for j in 0..strlen{
        let o_or_e:char;
        o_or_e = if j % 2 == 0 {
            'L'
        }else{
            'R'
        };
        result = if strs[j] == o_or_e{
            false
        }else{
            result
        };
    }
    if result {println!("Yes")} else{println!("No")};
}
