use crate::user::*;



#[derive(Debug)]
pub enum BankName {
    Mashreq,
    NBD,
    UsBank,
    GoodBank
}

#[derive(Debug)]

pub struct Bank<T: Work> {
    pub user: User<T>,
    pub bank_amount: u128,
    pub bank_name: BankName
}

pub fn work_very_hard<T: Work> (work: T) -> MoneyLevel {
    match work.work_hard() {
        MoneyLevel::Low => MoneyLevel::Medium,
        MoneyLevel::Medium => MoneyLevel::High,
        _ => MoneyLevel::Medium
    }
}

#[derive(Debug)]
pub enum Stamp {
    Approve,
    Deny
}


#[derive(Debug)]
pub struct Visa<'a, T: Work> {
    pub bank: &'a Bank<T>
}


impl<'a, T: Work> Visa<'a, T> {
    pub fn send_approval(&self) -> Stamp {
        if self.bank.bank_amount >= 1000 {
            Stamp::Approve
        } else {
            Stamp::Deny
        }
    }
}


pub fn fix<'a>(name: &'a str) -> &'a str {
    name.trim()
}