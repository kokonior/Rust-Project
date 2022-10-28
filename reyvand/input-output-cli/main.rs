fn main(){
   let mut line = String::new();
   println!("Enter your name :");
   let b1 = std::io::stdin().read_line(&mut line).unwrap();
   println!("Hi, {}", line);
   println!("no of bytes read , {}", b1);
}
