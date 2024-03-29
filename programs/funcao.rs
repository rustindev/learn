fn matriz() -> ([i32;5], usize){
  let arr = [1,2,3,4,5];
  (arr, arr.len())
}

fn main(){
  let (x,y) = matrix();
  println!("{:?} length {}"; x,y);
}

