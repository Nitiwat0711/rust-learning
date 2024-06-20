// Type of reference
// 1. Immutable Reference
// 2. Mutable Reference (add &)

// -------

// 1. Immutable Reference
fn main() {
    // let mut _x: i32 = 5;
    // let _r = &mut _x;

    // *_r +=1;
    // *_r -=3;

    // println!("Value of _x: {}", _x);
    // println!("Value of _r: {}", _r);

    // ----
    let mut account: BankAccount = BankAccount {
        owner: "Alice".to_string(),
        balance: 150.55,
    };

    //Immutable borrow to check the balance
    account.check_balance();

    // Mutable borrow to withdraw money
    account.withdraw(45.5);

    //Immutable borrow to check the balance
    account.check_balance();
}

struct BankAccount {
    owner: String,
    balance: f64,
}

impl BankAccount {
    fn withdraw(&mut self, amount: f64) {
        println!("Withdrawing {} from account by {}", amount, self.owner);
        self.balance -= amount;
    }
    fn check_balance(&self) {
        println!(
            "Account owned by {} has a balance of {}",
            self.owner, self.balance
        );
    }
}
