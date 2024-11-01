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

}
