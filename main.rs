fn main() {
    
    let output = reverse(98543);
    
    println!("The value of reversed is: {}", output);
    
}

fn reverse(x:i64) -> i64 {
    
    let mut input = x as i64;
    
    let mut reverse_var:i64 = 0;
    
    
    while input != 0 {
        
        let last_digit = input % 10;
        reverse_var = reverse_var * 10 + last_digit;
        input = input / 10;
    }
    
     return reverse_var;
}
