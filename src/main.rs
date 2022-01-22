fn main() {
    project_euler_08();
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

fn project_euler_07(){
    let mut prime_cnt = 0;
    let mut index = 1;
    loop {
        match is_prime(index) {
            true =>{
                prime_cnt+=1;
                if prime_cnt == 10001 {
                    break;
                }else{
                    index+=1;
                }
             },
            false => index+=1
        }
    }
    println!("{}", index);
}

fn project_euler_08(){
    let num_str = String::from("7316717653133062491922511967442657474235534919493496983520312774506326239578318016984801869478851843858615607891129494954595017379583319528532088055111254069874715852386305071569329096329522744304355766896648950445244523161731856403098711121722383113622298934233803081353362766142828064444866452387493035890729629049156044077239071381051585930796086670172427121883998797908792274921901699720888093776657273330010533678812202354218097512545405947522435258490771167055601360483958644670632441572215539753697817977846174064955149290862569321978468622482839722413756570560574902614079729686524145351004748216637048440319989000889524345065854122758866688116427171479924442928230863465674813919123162824586178664583591245665294765456828489128831426076900422421902267105562632111110937054421750694165896040807198403850962455444362981230987879927244284909188845801561660979191338754992005240636899125607176060588611646710940507754100225698315520005593572972571636269561882670428252483600823257530420752963450");
    let mut biggest:i64 = 0;
    for x in 0..num_str.len(){
        let stop =  if x+13 > num_str.len() {
                                let leftover = num_str.len() - x;
                                x+leftover
                            }else{
                                x+13
                            };

        let mut subslice = &num_str[x..stop];
        let mut temp :i64 = 1;
        for char in subslice.chars(){
            temp = temp * char.to_digit(10).unwrap() as i64;
        }
        if temp > biggest {
            biggest = temp;
        }
    }
    println!("{}", biggest);
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

