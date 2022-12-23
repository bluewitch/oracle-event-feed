fn main() {
  // Get a string from the user
  let mut input = String::new();
  println!("Enter a string:");
  std::io::stdin().read_line(&mut input).expect("Failed to read input");

  // Remove the newline character from the end of the string
  input.pop();

  // Print the string in reverse
  let reversed = input.chars().rev().collect::<String>();
  println!("Reversed string: {}", reversed);
}
