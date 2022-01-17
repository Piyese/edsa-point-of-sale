use std::{path::Path, fs::{OpenOptions, self}, io::Write};

use serde::{Deserialize, Serialize};

use super::errors::PosError;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Role {
    Supplier,
    Customer,
    Other
}

impl Role {
    /// Returns `true` if the role is [`Other`].
    ///
    /// [`Other`]: Role::Other
    pub fn is_other(&self) -> bool {
        matches!(self, Self::Other)
    }

    /// Returns `true` if the role is [`Supplier`].
    ///
    /// [`Supplier`]: Role::Supplier
    pub fn is_supplier(&self) -> bool {
        matches!(self, Self::Supplier)
    }

    /// Returns `true` if the role is [`Customer`].
    ///
    /// [`Customer`]: Role::Customer
    pub fn is_customer(&self) -> bool {
        matches!(self, Self::Customer)
    }
}
impl Default for Role {
    fn default() -> Self {
        Self::Other
    }
}

#[derive(Debug, Default, Clone, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Person { 
    pub role: Role,
    pub name: String,
    pub tel: String,
}
impl Person {
    /// Creates a new [`Person`].
    ///
    /// # Examples
    ///
    /// ```
    /// use edsa_pos::sale::people::Person;
    ///
    /// let result = Person::new(role, name, tel);
    /// 
    /// ```
    pub fn new(role:Role, name: String, tel: String)->Self {
        Self {role, name, tel}
    }
    /// Logs [`Person`] into a yaml file.
    pub fn log(self)->Result<(), PosError>{
        let path = Path::new("records/people");
        let people_log = [self];
        let people_log = serde_yaml::to_vec(&people_log)?;

        if path.exists(){
            let mut file=OpenOptions::new().append(true).open(path)?;
            let people_log=&people_log[4..];
            file.write_all(&people_log)?;
            Ok(())
        }else{
            let mut file=fs::File::create(path).expect("cant open file");
            file.write_all(&people_log)?;
            Ok(())
        }
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
    /// Logs [`Employee`] into a yaml file.
    pub fn log(self)->Result<(), PosError>{
        let path = Path::new("records/employees");
        let employee_log = [self];
        let employee_log = serde_yaml::to_vec(&employee_log)?;

        if path.exists(){
            let mut file=OpenOptions::new().append(true).open(path)?;
            let employee_log=&employee_log[4..];
            file.write_all(&employee_log)?;
            Ok(())
        }else{
            let mut file=fs::File::create(path).expect("cant open file");
            file.write_all(&employee_log)?;
            Ok(())
        }
    }
}
