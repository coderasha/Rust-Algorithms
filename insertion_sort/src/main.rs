fn main() {
    // let mut v = vec![8,6,7,3,9,2];
    let mut v = vec!["pupu","chanchi","ani","koko","kash","ham"];
insertion_sort(&mut v );
    println!("{:?}",v);
}
fn insertion_sort<T:Ord>(arr:&mut Vec<T>){
    for i in 1..arr.len(){
        let mut j = i;
        while j>0 && arr[j]< arr[j-1]{

            arr.swap(j,j-1);
            j=j-1 ;
        }




    }



  

}
