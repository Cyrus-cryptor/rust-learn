fn main() {
    let res = fibo(7);
    println!("Res is {res}");
}

fn for_fun() {
    for number in (2..9).rev() {
        println!("The value in for is {number}");
    }
    println!("End for ");
}

fn fibo(n: i32) -> i32 {
    if n <= 2 {
        return 1;
    }
    return fibo(n - 1) + fibo(n - 2);
}

// fn another_function() {
//     println!("This is another function");
// }

// fn another_function_with_param(x: i32) {
//     println!("The value is {x}");
// }

// fn print_labeled_measurement(value: u32, unit_label: char) {
//     println!("The measurement is:  {value}{unit_label}")
// }
