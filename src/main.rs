#[derive(Debug)]
struct Account {
    id: u32,
    balance: i32,
    holder: String,
}

impl Account {
    fn new(id: u32, holder: String) -> Self {
        Account {
            id, balance: 0, holder
        }
    }

    fn deposit(&mut self, amount: i32)  -> i32{
        self.balance += amount;
        self.balance
    }
    fn withdraw(&mut self, amount: i32) -> i32 {
        self.balance -= amount;
        self.balance
    }

    fn summary(&self) -> String {
        format!("Account holder: {} has a balance {}", self.holder, self.balance)
    }
}
#[derive(Debug)]
struct Bank {
    accounts: Vec<Account>,
}

impl Bank {
    fn new() -> Self {
        Bank {accounts: Vec::new()}
    }

    fn add_account(&mut self, account: Account) {
        self.accounts.push(account);
    }

    fn total_balance(&self) -> i32 {
        self.accounts.iter().map(|account| account.balance).sum()
    }

    fn summary(&self) -> Vec<String> {
        let vec = self.accounts
            .iter()
            .map(|account| account.summary())
            .collect::<Vec<String>>();
        vec
    }
}

fn print_account(account: &Account) {
    println!("{:#?}", account);
}

fn print_num_accounts(bank: &Bank) {
    println!("{:#?}", bank.accounts.len());
}

fn change_account(account: &mut Account) -> i32 {
    account.balance += 10;
    return account.balance;
}


fn main() {
    let mut bank = Bank::new();
    let mut account = Account::new(1, String::from("Maria"));
    //let account2 = Account::new(2, String::from("Dorian"));

    //print_num_accounts(&bank);
    //print_account(account_ref1);
    //print_account(account_ref2);
    //print_account(bank.accounts);
    //print_account(&account);


    let deposited_amount = account.deposit(100000);
    let withdraw_amount = account.withdraw(500);
    //println!("{}", account.summary());
    bank.add_account(account);
    println!("{:#?}", bank.summary());
    //println!("Total Account summary is {:#?}", bank.summary());
    println!("Total Bank Balance is {}", bank.total_balance());
    
    println!("{:#?}", bank);

}
