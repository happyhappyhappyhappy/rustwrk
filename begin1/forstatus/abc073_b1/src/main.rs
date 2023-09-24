use proconio::input;

fn main() {
    input!{
        n:usize,
        lr:[(usize,usize);n]
    }
    let mut sum:usize=0;
    for (x,y) in lr.iter(){
        // println!("{}-{}",x,y);
        // println!("{}-{}={}",y,x,y-x);
        sum = sum + ((y-x)+1);
    }
    println!("{}",sum);
}
