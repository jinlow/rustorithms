fn main() {
    let ar: [i32; 10] = [1, 2, 43, 70, 100, 200, 300, 500, 600, 1000];
    println!("{}\n", binary_search(&ar, 70).unwrap());
    let ar2: Vec<i32> = (0..1000000).collect();
    println!("{}", binary_search(&ar2, 70).unwrap());
}

fn binary_search(a: &[i32], item: i32) -> Option<i32> {
    println!("Searching for {}, in a slice of length {}", item, a.len());
    let mut low = 0;
    let mut high = a.len() - 1;
    let mut ticker = 0;
    while low <= high {
        ticker += 1;
        let mid = (low + high) / 2;
        println!("mid: {}", mid);
        let guess = a[mid];
        if guess == item {
            println!("You got it in {} guesses!", ticker);
            println!("Low: {} High: {} guess: {}", low, high, guess);
            return Some(item);
        }
        println!("Low: {} High: {} guess: {}", low, high, guess);
        low = if guess <= item { mid + 1 } else { low };
        high = if guess > item { mid - 1 } else { high };
    }
    return None;
}
