fn main() {
  // 3 2 1
  for number in (1..4).rev() {
      println!("{number}");
  }
  println!("{:->20}","-");
  // 1 2 3
  for number in 1..4 {
    println!("{number}");
  }
  println!("{:->20}","-");
  // a b c d
  for letter in 'a'..='d' {
    println!("{letter}");
  }
}