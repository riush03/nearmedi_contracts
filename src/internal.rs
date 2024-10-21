use crate::*;
// use near_workspaces::Contract;


//This will be include in the contract body
impl Contract {
    pub fn is_admin(&self) -> bool {
        env::current_account_id() == self.owner
    }

    pub fn is_user(&self) -> bool{
        self.users.contains(&env::predecessor_account_id())
    }

    pub fn is_owner_or_admin(&self) -> bool{
        self.is_user() || self.is_admin()
    }
}