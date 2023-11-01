use std::io;



fn reverse_string(input: &str) -> String
{
  let reversed : String = input.chars().rev().collect();

  reversed
}

fn main() {
    println!("Enter a string to reverse");

    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed to read line");

    input = input.trim().to_string();

    let reversed = reverse_string(&input);

    println!("Original: {}", input);
    println!("Reversed: {}", reversed);


}