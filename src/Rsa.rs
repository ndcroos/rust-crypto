
fn main() {
  let int n = 12345;
  let int eA = 5;
  let int eB = 13;
  let int c = 54321;
  
  decrypt(n, eA, eB, c);
  
  
}


fn decrypt(n : i32, eA : i32, eB : i32, c : i32){
  println!("Prime factors");
  
  let p = FindPrimeFactor(n);
  let q = n / p;
  let phi = (p - 1) * (q - 1);
  
  println!("p = " + p");
  println!("q = " + q");
  println!("phi = " + phi");
    
  println!("Bezout");
    
  println!("Calculate m");
}

fn chineseRemainder(){
    
    
}
    
fn remainder() -> i32 {
    
    
}
    
fn bezoutInverse() -> i32 {
    
    
    
}
