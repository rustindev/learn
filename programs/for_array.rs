fn main(){
  let ar = [1,2,3,4,5];
  for (i,value) in ar.iter().enumerate(){
    println!("[{}]:{}", i, value);
  }
}