use proconio::input;

fn main() {
    input!{
        cnt:usize,
        mut a:[i64;cnt],
    }
    let mut counter:usize = 0;
    let mut flug:usize = 1;
    loop {
        for j in 0..cnt{
            if (a[j] % 2)==0{
                a[j] = a[j] / 2;
            }else{
                flug=0;
            }
        }
        if flug == 0{
            break
        }else{
            counter=counter+1
        };
    }
    println!("{}",counter);
}
