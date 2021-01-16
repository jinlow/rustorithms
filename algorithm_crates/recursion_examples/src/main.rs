fn main() {
    println!("5 factorial is: {}", factorial(5));
    print_rectangle(1);
    print_rectangle(4);
    print_rectangle(8);
    let ar: [i32; 3] = [1, 2, 3];
    println!("Recursive Sum of {:?} is: {}", ar, recursive_sum(&ar));
    println!("Array length is: {} check: {}", arr_len(&ar), ar.len());
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

// Right now every element in the array is being copied
// so this would need to be made more efficient.
fn recursive_sum<T: Copy + std::ops::Add<Output = T>>(x: &[T]) -> T {
    if x.len() == 1 {
        return x[0];
    } else {
        return x[0] + recursive_sum(&x[1..]);
    }
}

// Count the number of items in an array
fn arr_len<T>(x: &[T]) -> usize {
    if x.is_empty() {
        return 0;
    } else {
        return 1 + arr_len(&x[1..]);
    }
}

// Print a rectangle to the terminal, given a side length
fn print_rectangle(height: usize) {
    print_diags(0, height);
    println!(" {}", rep_char(height * 2, "-"))
}

fn rep_char(n: usize, cr: &str) -> String {
    std::iter::repeat(cr).take(n).collect::<String>()
}

fn print_diags(level: usize, height: usize) {
    if level == height {
        return;
    } else {
        println!(
            " {}/{}\\",
            rep_char(height - (level + 1), " "),
            rep_char(level * 2, " ")
        );
        print_diags(level + 1, height);
    }
}
