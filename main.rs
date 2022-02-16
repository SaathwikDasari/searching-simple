use std::io;
use std::io::{Write};

fn main() {
    let arr: [&str; 6] = ["Saathwik", "Sai Charan", "Abhinav", "Cherry", "Chandra", "Chandra"];

    let mut user_input = String::new();

    let mut ans: u32 = 0;

    print!("What do you want to find out?: ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut user_input).unwrap();

    for i in arr.iter() {
        if i == &user_input.trim() {
            ans += 1;
        }
    }

    println!("Found {} matches!", ans);
    
}
