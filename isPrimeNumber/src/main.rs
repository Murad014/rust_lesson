
fn main() {
    let input_number = 1;

    println!("{} is {}", input_number,
             if is_prime(input_number) {"prime number!"} else {"not prime number!"})
}



fn is_prime(number: u32) -> bool{
    if number == 1 {return false;}

    let number_float :f32 = {number as f32}.sqrt();
    let number_sqrt = number_float as u32;

    for i in 2..number_sqrt {
        if number % i == 0 {
            return false;
        }
    }


    true
}