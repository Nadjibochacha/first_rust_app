use std::collections::HashMap;



pub fn main1() {
    let bool_var = true;
    println!("let {}",bool_var); //print a var
    const NUM:i32 = 343; // difine a const and specify a type
    println!("cost {}",NUM);
    let numbers : [i32;6] = [1,3,4,56,86,21]; //array of ints
    println!("array :{:?}",numbers);
    let strings:[&str;3] = ["nadjib","ahmed","ali"];
    println!("array of strings :{:?}",strings);
    let tuple = ("nadjib",22,true); //tuple contian different types of data with a fixed size
    println!("tuple :{:?}",tuple);
    let slice = &[1,2,3,4,5,6]; 
    // vectors
    let mut  _vec = vec!["ahmed", "ali", "karim"];
    let mut _vec2 : Vec<i32> = vec![1,2,3,4,56,7,8,9];
    _vec.push("nadjib");
    _vec2.pop();
    // hashmap
    let mut scores = HashMap::new();
    scores.insert(12, "nadjib");
    scores.insert(1, "ali");
    scores.insert(2, "nooh");
}