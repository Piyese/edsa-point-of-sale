
use serde::{Deserialize, Serialize};

use super::accounts::Owe;



#[derive(Debug, Default, Clone, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Person {
    pub name: String,
    pub tel: String,
}
impl Owe for Person{}
impl crate::LogPartial for Person{}

impl Person {
    /// Creates a new [`Person`].
    ///
    /// # Examples
    ///
    /// ```
    /// use edsa_pos::sale::people::Person;
    ///
    /// let result = Person::new(name, tel);
    /// 
    /// ```
    pub fn new(name: String, tel: String)->Self {
        Self {name, tel}
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Sex {
    Male,
    Female,
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Employee {
    pub name: String,
    pub sex: Sex,
    pub active: bool,
    pub tel: String,
}

impl crate::LogPartial for Employee{}

impl Employee {
    /**
    Creates a new [`Employee`].

    # Examples

    ```
    use edsa_pos::sale::people::Employee;

    let result = Employee::new(name, sex, active, tel);

    ```
    */
    pub fn new(name: String, sex: Sex, active: bool, tel: String)->Self {
        Self {name, sex, active, tel}
    }
}
