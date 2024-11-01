fn main() {
 let v1 =vec![1,2,3];
 let v1_iter = v1.iter();

 for val in v1_iter{
    println!(":{val}");

 }


 let mut nums = vec![1,2,3];
 let iter = nums.iter_mut();

 for value in iter{
    *value = *value+1;
 }   

 println!("{:?}",nums);


 //iteratin usng .next

 let mut _iter = nums.iter_mut();
 while let Some(val) = _iter.next(){
    println!("{}",val);
 }

 //intoiter
 let intoiter_iter = nums.into_iter();
 for val in intoiter_iter{
    println!("{}",val);
 }
 
 }
