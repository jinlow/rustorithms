fn main() {
    println!("5 factorial is: {}", factorial(5));
    print_rectangle(1);
    print_rectangle(4);
    print_rectangle(8);
}

// The factorial function:
// The "Hello World" of recursion
fn factorial(x: i32) -> i32 {
    if x == 1 {
        return 1;
    } else {
        return x * factorial(x - 1);
    }
}

fn rep_char(n: usize, cr: &str) -> String {
    std::iter::repeat(cr).take(n).collect::<String>()
}

fn print_rectangle(height: usize) {
    for i in 0..height {
        let x = if i == 0 { 0 } else { 1 * i };
        println!(
            " {}/{}\\",
            rep_char(height - (i + 1), " "),
            rep_char((i + (i * 2)) - x, " ")
        );
    }
    println!(" {}", rep_char(height * 2, "-"))
}
