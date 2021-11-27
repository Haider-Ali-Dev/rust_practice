/// Make a User here
#[derive(Debug)]

pub struct User<T: Work> {
    pub user_work: T,
    pub name: String,
    pub age: u32 
}




impl<T: Work> User<T> {
    pub fn new(name: &str, age: u32, user_work: T) -> Self {
        Self {
            user_work,
            name: name.to_owned(),
            age
        }        
    }
}

/// MoneyLevel for Work trait

#[derive(Debug)]
pub enum MoneyLevel {
    Low, 
    Medium,
    High
}


/// Work trait 
pub trait Work {
    /// **work_hard** function will work and then you
    /// get a `MoneyLevel` 
    fn work_hard(&self) -> MoneyLevel;
}