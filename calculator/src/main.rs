use std::io;

fn main() {
    initial_print();

    loop {
        let mut a_str = String:: new();
        let mut b_str = String:: new();
        let mut operation = String:: new();

        println!("Input a:");
        read_input(&mut a_str);

        println!("Input b: ");
        read_input(&mut b_str);

        println!("Select operation (+ - * /):");
        read_input(&mut operation);

        let (a, b) = convert_a_and_b_to_int(a_str, b_str);

        let calc_res = calc(a, b, operation);
        println!("Result: {}", calc_res);
    }

}

fn calc(a: i32, b: i32, operation: String) -> i32{
    let operation = operation.trim();

    if operation == "+" {
        a + b
    } else if operation == "-" {
        a - b
    } else if operation == "*"{
        a * b
    }
    else {
        a / b
    }
}

fn read_input(a: &mut String) {
    io::stdin()
        .read_line(a)
        .expect("Reading 'a' failed.");
}

fn convert_a_and_b_to_int(a: String, b: String) -> (i32, i32){
    let a: i32 = a.trim()
        .parse()
        .expect("Failed parse 'a'");

    let b: i32 = b.trim()
        .parse()
        .expect("Failed parse 'b'");

    (a, b)
}

fn initial_print(){
    println!("******** Calculate two number ********");
}
