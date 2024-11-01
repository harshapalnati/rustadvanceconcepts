use std::collections::HashMap;

fn main() {
    let mut users = HashMap::new();
    users.insert(String::from("a"),22);
    users.insert(String::from("b"),33);

    let val = users.get("a");

    match val {
        Some(age) =>println!("{}",age),
        None=>println!("user not found"),
    }
   
}
