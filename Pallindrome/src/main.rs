// fn main() {

//     let n:u128= 121;
//     is_pallindrome(&mut n);
//     // println!("the number is {}",);
// }
// fn is_pallindrome(n:&mut u128) {
//     let (mut r, mut s):(u128,u128) =
//     (0,0);
//    while n > 0{
//       r = n % 10;
//      s = r + s*10 ;
//      n = n/10;

//    }
//    if n==s{
//        println!("Pallindrome");
//    }else{
//    println!("Not Pallindrome");
//    }
// }

// use std::io;

// fn main() {

// let mut x = String::new();
// io::stdin().read_line(&mut x).expect("failed to read input.");
// let y = x.chars().rev().collect::<String>();

// println!("x:{} y{}",x,y);


// if x !=y {
//     println!("Not Palindrome");
// } else {
//     println!(" Palindrome");
// }
// }

fn main(){

let x= "pupu";
if x.to_string() == x.to_string().chars().rev().collect::<String>(){

println!("Pallindrome");

}else{
    println!("Not Pallindrome");
}

}



---------------------------------------------------------------------------------------
fn main(){

    is_pallindrome(1232);
 }
 fn is_pallindrome(x:i32){
     let mut c:i32;
     let (mut r,mut s):(i32,i32)=(0,0);
     c=x;
 
     while c>0{
         r= c%10;
         s = r+ s*10;
         c = c/10;
 
        
     }
     println!("s:{}",s);
     println!("x:{}",x);
 
     if s==x{
         println!("pallin");
     }else{
         println!("not pallind");
     }
 
 }

