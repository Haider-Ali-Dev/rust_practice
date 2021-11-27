use helpers::user::{ MoneyLevel, Work, User };
use helpers::bank::{ Bank, BankName, Visa};
struct Build;
struct Dance;

impl Work for Build {
    fn work_hard(&self) -> MoneyLevel {
        use MoneyLevel::*;
        Medium
    }
}

impl Work for Dance {
    fn work_hard(&self) -> MoneyLevel {
        use MoneyLevel::*;
        High
    }
}

fn main() {
    let user_3 = Bank {
        user: User { name: "Haider".to_owned(),  age: 17, user_work: Build},
        bank_amount: 1002,
        bank_name: BankName::Mashreq  
    };
    let user_approval = Visa {
        bank: &user_3
    };

    println!("{:?}", user_approval.send_approval());

}