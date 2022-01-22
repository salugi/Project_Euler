fn main() {
    project_euler_06();
}

fn project_euler_01(){
    let mut sum = 0;
    (0..1000).for_each(|x|{if x%3==0||x%5==0 {sum += x}});
    println!("{:?}", sum);
}
fn project_euler_02(){
    let mut temp1 = 0;
    let mut temp2 = 1;
    let mut sum =0;
    loop {
        let temp3 = temp1+temp2;
        temp1 = temp2;
        temp2 = temp3;
        if temp3 > 4000000 {
            sum += if temp3%2==0{temp3}else{0};
            break;
        }else{
            sum += if temp3%2==0{temp3}else{0};
        }
    }
    println!("{:?}", sum);
}

fn project_euler_03(){
    let smoke : f64 = 600851475143_f64;
    let limit = smoke.sqrt() as i64 + 1;
    let mut lpf = 0;
    for x in (0..limit).rev() {
        if 600851475143 % x == 0 {
            if is_prime(x) {
                lpf = x;
                break;
            }
        }
    }
    println!("{:?}", lpf);
}



fn project_euler_04(){
    let smoke = 998001;
    let mut palindrome = 0;
    let mut should_break = false;
    for x in (10000..smoke).rev() {
        let str_1 = x.to_string();
        if str_1 == str_1.chars().rev().collect::<String>() {
            let mut factor_vec:Vec<i32> = vec![];
            for i in 99..1000{
                if x % i == 0 {
                    factor_vec.push(i);
                }
            }
            for factor in factor_vec {
                let div = (x/factor).to_string();
                if div.len() == 3{
                    should_break = true;
                }

            }
            if should_break{
                palindrome = x;
                break;
            }
        }
    }
    println!("{:?}", palindrome);
}

fn project_euler_05(){
    let mut index = 20;

    loop {
        let mut factor_vec :Vec<i32>=vec![];
        for i in 1..21{
            if index % i == 0 {
                factor_vec.push(i);
            }else{
                break;
            }
        }
        if factor_vec.len() == 20 {
            break;
        }else{
            index += 20;
        }
    }
    println!("{:?}", index);
}

fn project_euler_06(){
    let hundo_a = (1..101 as i32).into_iter().reduce(|a, b| a + b.pow(2)).unwrap();
    let hundo_b = (1..101 as i32).into_iter().reduce(|a, b| a + b).unwrap().pow(2);

    if hundo_a - hundo_b < 0 {
        println!("{}", hundo_b - hundo_a);
    }else {
        println!("{}", hundo_a - hundo_b);
    }
}

/*

helpers

 */
fn is_prime(n: i64) -> bool {
    if n <= 3 {
        return n > 1;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    if n < 25 {
        return true;
    }
    let mut i: i64 = 5;
    while i.pow(2) <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    return true;
}

