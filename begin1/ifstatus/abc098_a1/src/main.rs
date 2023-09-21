use proconio::input;

fn main(){
    input!{
        a:i64,
        b:i64,
    }
    let plus:i64=a+b;
    let minus:i64=a-b;
    let cross:i64=a*b;
    let mut answer:i64=std::i64::MIN;

    answer = if plus > answer{
        plus
    }else{
        answer
    };

    answer =  if minus > answer{
        minus
    }else{
        answer
    };

    answer = if cross > answer{
        cross
    }else{
        answer
    };
    println!("{}",answer);
}
