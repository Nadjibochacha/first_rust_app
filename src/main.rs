mod types; //import a file
//the main process
fn main() {
    // types::main1(); //call fn of file
    let s = String::from("nadjib");
    print!("length: {}", length_string(&s))
}

fn length_string(s:&String) -> usize {
    s.len()
}
