/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 */
 #[cfg(test)]
 mod tests {
    //  use near_sdk::AccountId;
 
    //  use crate::{Contract, DoctorInput};
 
    //  fn get_contract() -> Contract {
    //      let bob: AccountId = "bob.near".parse().unwrap();
    //      Contract::init(bob, None)
    //  }
    use super::*; // Import everything from the parent module
    use near_workspaces::Contract;
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::testing_env;
    

    fn get_contract() -> Contract {
        let mut context = VMContextBuilder::new();
        context.predecessor_account_id(accounts(0));
        testing_env!(context.build());
        Contract::default() // Initialize your contract
    }
 
     fn add_doctor_to_contract(contract: &mut Contract) {
         contract.add_doctor(DoctorInput {
            title: "Dr.".to_string(),
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            gender: "Male".to_string(),
            designation: "Cardiologist".to_string(),
            last_work: "Hospital A".to_string(),
            email: "john.doe@example.com".to_string(),
            college_name: "Medical College".to_string(),
            college_id: "MC123".to_string(),
            joining_year: 2010,
            end_year: 2015,
            specialization: "Heart Surgery".to_string(),
            registration_id: "REG123".to_string(),
            college_address: "Meru 567".to_string(),
            account_id: "bob.near".parse().unwrap(),
            profile_pic: "profile.jpg".to_string(),
            bio: "Experienced doctor in cardiology.".to_string(),
         });
 
         contract.add_doctor(DoctorInput {
            title: "Dr.".to_string(),
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            gender: "Male".to_string(),
            designation: "Cardiologist".to_string(),
            last_work: "Hospital A".to_string(),
            email: "john.doe@example.com".to_string(),
            college_name: "Medical College".to_string(),
            college_id: "MC123".to_string(),
            joining_year: 2010,
            end_year: 2015,
            specialization: "Heart Surgery".to_string(),
            registration_id: "REG123".to_string(),
            college_address: "Meru 567".to_string(),
            account_id: "bob.near".parse().unwrap(),
            profile_pic: "profile.jpg".to_string(),
            bio: "Experienced doctor in cardiology.".to_string(),
         });
     }
 
     #[test]
     fn get_doctor_by_id() {
         let mut contract = get_contract();
         add_doctor_to_contract(&mut contract);
 
         let doctor = contract.get_doctor_id(1).unwrap();
 
         assert_eq!(doctor.id, 1);
     }
 

 }

 //implementation of the nft_transfer method. This transfers the NFT from the current owner to the receiver. 
#[payable]
fn nft_transfer(
    &mut self,
    receiver_id: AccountId,
    token_id: TokenId,
    memo: Option<String>,
) {
    //assert that the user attached exactly 1 yoctoNEAR. This is for security and so that the user will be redirected to the NEAR wallet. 
    assert_one_yocto();
    //get the sender to transfer the token from the sender to the receiver
    let sender_id = env::predecessor_account_id();

    //call the internal transfer method
    self.internal_transfer(
        &sender_id,
        &receiver_id,
        &token_id,
        memo
    );
}