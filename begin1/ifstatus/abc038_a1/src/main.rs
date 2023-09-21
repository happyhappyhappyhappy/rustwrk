use proconio::input;

fn main() {
    input!{
        // s: proconio::marker::Chars,
        s : proconio::marker::Chars,
    }
    let x:usize=s.len();
    let tea:char=s[x-1];
    if tea == 'T'{
        println!("YES")
    }else{
        println!("NO")
    };
}
