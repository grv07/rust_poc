fn main() {
    let  input = 600851475143;
    let t = prime_fac(input);
    println!("{}", t);
}

fn prime_fac(mut num: usize) -> usize {
    let mut result = Vec::new();
    while num % 2 == 0 {
        result.push(2);
        num = num / 2;
    }
    
    let limit = (num as f64).sqrt() as usize + 1;
    for i in (3..limit).step_by(2) {
        while num % i == 0 {
            result.push(i);
            num = num / i;
        } 
    }
    
    if num > 2 {
        result.push(num);
    }
    
    *result.iter().max().unwrap()

}
