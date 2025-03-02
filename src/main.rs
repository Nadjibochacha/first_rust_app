mod types; //import a file
//the main process
fn main() {
    // types::main1(); //call fn of file
    let s = String::from("nadjib");
    // print!("length: {}", length_string(&s))
    let mut  account = BankAccount{
        owner: "Ahmed".to_string(),
        balance: 34250.00,
    };
    account.checkAccount();
    account.withdraw(200.40);
    account.checkAccount();
}

fn length_string(s:&String) -> usize {
    s.len()
}

// borrow and references:

// like you created a type or data structure and added its funcs
struct BankAccount {
    owner: String,
    balance: f64
}

impl BankAccount {
    fn checkAccount(&self) {
        println!("{} has ${} in his account.",self.owner,self.balance)
    }
    fn withdraw(&mut self, amount: f64) {
        println!("withdrawing ${} from the account of {}.", amount, self.owner);
        self.balance -= amount
    }
}