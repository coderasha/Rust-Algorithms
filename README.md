Bubble Sort



**for both string and integers**

fn main(){

let x= "pupu";
if x.to_string() == x.to_string().chars().rev().collect::<String>(){

println!("Pallindrome");

}else{
    println!("Not Pallindrome");
}

}

  
  **for integers**
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


  
