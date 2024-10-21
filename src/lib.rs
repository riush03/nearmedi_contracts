use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env,log ,near, require,near_bindgen, AccountId, NearToken, PanicOnDefault, Promise};
use near_sdk::store::{IterableSet,LookupSet,Vector,UnorderedSet,UnorderedMap};
// use serde_json::json

pub mod internal;
pub mod utils;
pub use crate::utils::*;

#[near(serializers = [json, borsh])]
#[derive(Clone,PartialEq)]
pub enum AppointmentStatus {
    Pending,
    Completed,
}

#[near(serializers = [json, borsh])]
#[derive(Clone)]
pub struct PatientInput {
    id:u32,
    title: String,
    first_name: String,
    last_name: String,
    gender: String,
    condition: String,
    phone: u32,
    email: String,
    dob: u32,
    city: String,
    address: String,
    doctor: String,
    profile_pic: String,
    account_id: AccountId,
    message: String,
}

#[near(serializers = [json, borsh])]
#[derive(Clone)]
pub struct Patient {
    id: u32,
    title: String,
    first_name: String,
    last_name: String,
    gender: String,
    condition: String,
    phone: u32,
    email: String,
    dob: u32,
    city: String,
    address:String,
    doctor: String,
    profile_pic: String,
    account_id: AccountId,
    message: String,
    medical_history: Vec<String>,
    bought_medicine: Vec<i32>,
}

#[near(serializers = [json, borsh])]
pub struct DoctorInput {
    title: String,
    first_name: String,
    last_name: String,
    gender: String,
    designation: String,
    last_work: String,
    email: String,
    college_name: String,
    college_id: String,
    joining_year: u32,
    end_year: u32,
    specialization: String,
    registration_id: String,
    college_address: String,
    profile_pic: String,
    account_id: AccountId,
    bio: String,
}

#[near(serializers = [json, borsh])]
#[derive(Clone)]
pub struct Doctor {
    id: u32,
    title: String,
    first_name: String,
    last_name: String,
    gender: String,
    designation: String,
    last_work: String,
    email: String,
    college_name: String,
    college_id: String,
    joining_year: u32,
    end_year: u32,
    specialization: String,
    registration_id: String,
    college_address: String,
    account_id: AccountId,
    profile_pic: String,
    bio: String,
    appointment_counts: i32,
    successful_treaments: i32,
    is_approved: bool,
}

#[near(serializers = [json, borsh])]
#[derive(Clone)]
pub struct Medicine {
    id: u32,
    doctor_id: u32,
    name: String,
    brand: String,
    manufacturer: String,
    manufacturing_date: String,
    expiry_date: String,
    company_email: String,
    discount: u128,
    manufacturer_address: String,
    price: u128,
    quantity: u128,
    current_location: String,
    phone_no: u64,
    image: String,
    description: String,
    availability: bool,
}

#[near(serializers = [json, borsh])]
#[derive(Clone)]
pub struct Prescription {
    id: u32,
    medicine_id: u32,
    patient_id: u32,
    doctor_id: u32,
    date: u64,
}

#[near(serializers = [json, borsh])]
#[derive(Clone)]
pub struct Appointment {
    id: u32,
    patient_id: u32,
    doctor_id: u32,
    from: String,
    to: String,
    appointment_date: String,
    condition: String,
    status: AppointmentStatus,
    message: String,
    is_open:bool,
}

#[near(serializers = [json, borsh])]
pub struct Message{
    account_id: AccountId,
    timestamp: u64,
    message: String,
}



#[near(serializers = [json, borsh])]
#[derive(Clone)]
pub struct Order {
     medicine_id: u32,
     price: u128,
     payment_amount: u64,
     quantity: u128,
     patient_id: u64,
     date: u64,
}

#[near(serializers = [json, borsh])]
#[derive(Clone)]
pub struct Notification {
    account_id: AccountId,
    message: String,
    timestamp: u64,
}

#[near(contract_state)]
// #[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Contract {
    owner: AccountId,
    users: IterableSet<AccountId>,
    patients: Vector<Patient>,
    no_of_patients: u128,
    doctors: Vector<Doctor>,
    no_of_doctors: u128,
    drugs: Vector<Medicine>,
    no_of_drugs: u128,
    prescriptions: Vector<Prescription>,
    no_of_prescriptions: u128,
    appointments: Vector<Appointment>,
    no_of_appointments: u128,
    notifications: Vector<Notification>,
    no_of_notifications: u128,
    orders: Vector<Order>,
    appointment_fee: u128,
    registration_fee: u128,
}

// Implement the default method for Contract, initializing all collections
impl Default for Contract {
    fn default() -> Self {
        Self {
            owner: "medinear.testnet".parse().unwrap(),
            users: IterableSet::new(b"s"),
            patients: Vector::new(b"p"),
            no_of_patients: 0,
            doctors: Vector::new(b"d"),
            no_of_doctors: 0,
            drugs: Vector::new(b"h"),
            no_of_drugs: 0,
            prescriptions: Vector::new(b"p"),
            no_of_prescriptions: 0,
            appointments: Vector::new(b"e"),
            no_of_appointments: 0,
            notifications: Vector::new(b"k"),
            no_of_notifications: 0,
            orders: Vector::new(b"o"),
            appointment_fee: 42_000_000_000,
            registration_fee:42_000_000_000,
        }
    }
}

#[near]
impl Contract {
    #[init]
    pub fn init(owner: AccountId,users: Option<Vec<AccountId>>) -> Self {
        assert!(!env::state_exists(),"Already initialized");

        Self{
            owner,
            users: account_vec_to_set(
                if users.is_some() {
                    users.unwrap()
                } else {
                    vec![]
                },
                b"s",
            ),
            patients: Vector::new(b"p"),
            no_of_patients: 0,
            doctors: Vector::new(b"d"),
            no_of_doctors: 0,
            drugs: Vector::new(b"h"),
            no_of_drugs: 0,
            prescriptions: Vector::new(b"p"),
            no_of_prescriptions: 0,
            appointments: Vector::new(b"e"),
            no_of_appointments: 0,
            notifications: Vector::new(b"k"),
            no_of_notifications: 0,
            orders: Vector::new(b"o"),
            appointment_fee: 42_000_000_000,
            registration_fee:42_000_000_000,
        }
    }

    pub fn add_notification(&mut self, user_address: AccountId, message: String, ) {
        let timestamp = env::block_timestamp();

        let notification = Notification {
            account_id: user_address.clone(),
            message: message,
            timestamp: timestamp, 
        };

        // Store the notification in the vector
        self.notifications.push(notification);

        env::log_str(&format!("{} {} {}", user_address,"Notification sent to {} at {}", timestamp));
    }


    // pub fn get_notifications(&self) -> Vec<Notification>{
    //     return self.notifications
    //                 .iter()  // Iterate over references to `Prescription`
    //                 .cloned() // Clone the actual `Prescription`, not just the reference
    //                 .collect() /
    //    }

    pub fn add_medicine(
        &mut self,
        id: u32,
        doctor_id: u32,
        name: String,
        brand: String,
        manufacturer: String,
        manufacturing_date: String,
        expiry_date: String,
        company_email: String,
        discount: u128,
        manufacturer_address: String,
        price: u128,
        quantity: u128,
        current_location: String,
        phone_no: u64,
        image: String,
        description: String,
    ) {
        assert!(
            self.is_admin(),
            "Only the  admins can call this method"
        );
        let medicine = Medicine {
            id: id.clone(),
            doctor_id: doctor_id,
            name: name,
            brand: brand,
            manufacturer: manufacturer,
            manufacturing_date: manufacturing_date,
            expiry_date: expiry_date,
            company_email: company_email,
            discount: discount,
            manufacturer_address: manufacturer_address,
            price: price,
            quantity: quantity,
            current_location: current_location,
            phone_no: phone_no,
            image: image,
            description: description,
            availability: true,
        };

        self.drugs.push(medicine);
        self.no_of_drugs += 1;
    }

    //========== End of Medicine =======

    //========== Doctor =========----
    pub fn add_doctor(&mut self,id: u32,doctor: DoctorInput) {

        let doctor = Doctor {
            id: id.clone(),
            title:doctor.title,
            first_name: doctor.first_name,
            last_name: doctor.last_name,
            gender: doctor.gender,
            designation: doctor.designation,
            last_work: doctor.last_work,
            email: doctor.email,
            college_name: doctor.college_name,
            college_id: doctor.college_id,
            joining_year: doctor.joining_year,
            end_year: doctor.end_year,
            specialization:doctor.specialization,
            registration_id: doctor.registration_id,
            college_address: doctor.college_address,
            account_id: doctor.account_id,
            profile_pic: doctor.profile_pic,
            bio: doctor.bio,
            appointment_counts: 0,
            successful_treaments: 0,
            is_approved: false,
        };

        self.doctors.push(doctor);
        self.no_of_doctors += 1;
    }

    pub fn approve_doctor(&mut self, id: u32) {
        assert!(
            self.is_admin(),
            "Only the admins can call this method"
        );
    
        let mut doctor:Doctor = self.doctors.get(id).expect("Doctor not found").clone();
    
        doctor.is_approved = true;
    
        self.doctors.push(doctor.clone());
    
        env::log_str(&format!("Doctor with ID {} has been approved", id));
    
        self.add_notification(
            doctor.account_id.clone(),
            format!("Your account has been approved. Welcome to the platform!"),
        );
    }
    

    //Update by the doctor
    pub fn update_patient_medical(&mut self,id: u32,new_medical_history: String) {
        let caller_id = env::predecessor_account_id();
        assert!(self.is_doctor(caller_id), "Only the assigned doctor can update the medical history.");

        let mut patient: Patient = self.patients.get(id).expect("Doctor not found").clone();

        patient.medical_history.push(new_medical_history);

        self.patients.push(patient.clone());

        env::log_str(
            format!(
                "Patient medical history updated by doctor or admin for patient_id: {}",
                id
            )
            .as_str(),
        );
    }

    pub fn complete_appointment(&mut self, id: u32, patient_id: u32)  {
        // Retrieve the appointment from storage
        let mut appointment: Appointment  = self.appointments.get(id)
            .expect("Appointment not found").clone();

        // Verify the appointment belongs to the specified patient
        assert_eq!(appointment.patient_id, patient_id, "Appointment does not belong to the specified patient");

        // Check if the appointment is already completed
        assert!(appointment.status == AppointmentStatus::Completed, "Appointment is already completed");

        appointment.status = AppointmentStatus::Completed;
    }

    pub fn prescribe_medicine(&mut self,id:u32, medicine_id: u32, patient_id: u32,doctor_id:u32) {
        let caller_id = env::predecessor_account_id();
        assert!(self.is_doctor(caller_id), "Only doctors can prescribe medicine.");

        let prescription = Prescription {
            id: id,
            medicine_id:medicine_id,
            patient_id: patient_id,
            doctor_id: doctor_id,
            date: env::block_timestamp(),
        };

        self.prescriptions.push(prescription);
        self.no_of_prescriptions += 1;

        self.add_notification(env::predecessor_account_id(), "You have successfully added medicine.".to_string());
    }

    fn is_doctor(&self, account_id: AccountId) -> bool {
        // Implement logic to verify if the account_id belongs to a doctor
        self.doctors.iter().any(|doctor| doctor.account_id == account_id)
    }

    //======== End Of Doctor
    //===========  Patient
    pub fn add_patient(&mut self,patient: PatientInput) {

        let patient = Patient {
            id: patient.id,
            title: patient.title,
            first_name: patient.first_name,
            last_name: patient.last_name,
            gender: patient.gender,
            condition: patient.condition,
            phone: patient.phone,
            email: patient.email,
            dob: patient.dob,
            city: patient.city,
            address: patient.address,
            doctor: patient.doctor,
            profile_pic: patient.profile_pic,
            account_id: patient.account_id,
            message: patient.message,
            medical_history: vec![],
            bought_medicine: vec![],
        };

        self.patients.push(patient);
        self.no_of_doctors += 1;

        log!("Patient was registered successfully!");
    }

    pub fn book_appointment(&mut self,id:u32,patient_id: u32, doctor_id: u32, from: String, to: String, appointment_date: String, condition: String, message: String) {

        let appointment = Appointment {
            id: id,
            patient_id: patient_id,
            doctor_id: doctor_id,
            from: from,
            to: to,
            appointment_date: appointment_date,
            condition: condition,
            status: AppointmentStatus::Pending,
            message: message,
            is_open: true,
        };

        self.appointments.push(appointment);
        self.no_of_appointments += 1;

        
        self.add_notification(env::predecessor_account_id(), "You have successfully booked an appointment".to_string());

    }

    #[payable]
    pub fn buy_medicine(&mut self, medicine_id: u32, quantity: u128, patient_id: u32) -> Promise {
        // Retrieve the medicine details
        let medicine = self.get_medicine_by_id(medicine_id).expect("Medicine not found");
    
        // Check if the quantity is valid (should be greater than 0)
        assert!(quantity > 0, "Quantity must be greater than 0.");
    
        // Calculate the total price
        let total_price = medicine.price * quantity as u128;
    
        Promise::new(self.owner.clone()).transfer(NearToken::from_yoctonear(total_price.try_into().unwrap()))
    }
    

    // End of patient
    // Admin

    //Update by Admin only
     pub fn update_registration_fee(&mut self, new_fee: u128) {
        assert!(
            self.is_admin(),
            "Only the owner(patient) and admins can call this method"
        );
        self.registration_fee = new_fee;
        env::log_str(&format!("Registration fee updated to {}", new_fee));
    }

    // Function to update the appointment fee
    pub fn update_appointment_fee(&mut self, new_fee: u128) {
        assert!(
            self.is_admin(),
            "Only the owner(patient) and admins can call this method"
        );
        self.appointment_fee = new_fee;
        env::log_str(&format!("Appointment fee updated to {}", new_fee));
    }


    // Function to update the admin address
    pub fn update_admin_address(&mut self, new_admin: AccountId) {
        assert!(
            self.is_admin(),
            "Only the owner(patient) and admins can call this method"
        );
        self.owner = new_admin.clone();
        env::log_str(&format!("Admin address updated to {}", new_admin));
    }

    //======== End Of Admin
    //=========  Get patient data
    pub fn get_all_patient_orders(&self) -> Vec<Order> {
        self.orders
            .iter()  // Iterate over references to `Order`
            .cloned() // Clone each `Order` to get owned values
            .collect() // Collect into a Vec<Order>
    }
    

    // Retrieve all prescription details
    pub fn get_all_prescription_details(&self) -> Vec<Prescription> {
        self.prescriptions
            .iter()  // Iterate over references to `Prescription`
            .cloned() // Clone the actual `Prescription`, not just the reference
            .collect() // Collect into a Vec<Prescription>
    }
    

    pub fn get_all_registered_patients(&self) -> Vec<Patient> {
        self.patients
            .iter()     // Iterate over the vector of patients
            .cloned()   // Clone the Patient objects to return owned values
            .collect()  // Collect into a Vec<Patient>
    }
    
    pub fn get_patient_id(&self, patient_id: u32) -> Option<Patient> {
        // Assuming you have a patients collection to look up the patient by ID
        self.patients.iter().find(|patient| patient.id == patient_id).cloned()
    }

    pub fn get_patient_appointment(&self, patient_id: u32) -> Vec<Appointment> {
        let appointments: Vec<Appointment> = self
            .appointments
            .iter() // Iterate over the appointments
            .filter(|appointment| appointment.patient_id == patient_id) // Filter by patient ID
            .cloned() // Clone the appointment to return ownership
            .collect(); // Collect the results into a Vec<Appointment>
    
        appointments
    }
    

    pub fn get_patient_medical_history(&self, patient_id: u32) -> Vec<String> {
        // Retrieve the patient by ID
        let patient = self.get_patient_id(patient_id).unwrap(); // Assuming you have this method
    
        // Return the patient's medical history (Vec<String>)
        patient.medical_history.clone() // Clone to return ownership
    }
    
    pub fn get_patient_appointment_history(&self, patient_id: u32) -> Vec<Appointment> {
        self.appointments.iter()
            .filter(|appointment| appointment.patient_id == patient_id)
            .cloned()
            .collect()
    }

    pub fn get_bought_medicine_by_patient(&self, patient_id: u32) -> Vec<Medicine> {
        let patient = self.get_patient_id(patient_id); // Assuming you have this method

        // Collect medicines based on the IDs stored in `bought_medicine`
        patient.unwrap().bought_medicine.iter()
            .filter_map(|&medicine_id| self.get_medicine_by_id(medicine_id as u32)) // Assuming this method retrieves Medicine by ID
            .collect()
    }

    pub fn get_all_appointments(&self) -> Vec<Appointment>{
        self.appointments.iter().map(|appointment| appointment.clone()).collect()
    }

    // Get doctors data
    pub fn get_all_doctors_data(&self) -> Vec<Doctor>{
        self.doctors.iter().map(|doctor | doctor.clone()).collect()
    }

    pub fn get_approved_doctors(&self) -> Vec<Doctor> {
        self.doctors.iter().filter(|doctor| doctor.is_approved).cloned().collect()
    }



    pub fn get_doctor_details(&self, doctor_id: u32) -> Option<Doctor> {
        for doctor in self.doctors.iter() {
            if doctor.id == doctor_id {
                return Some(doctor.clone());
            }
        }
        None
    }


    pub fn get_doctor_appointment_historys(&self, doctor_id: u32) -> Vec<Appointment>{
        let mut history: Vec<Appointment> = Vec::new(); // Initialize an empty vector for the appointment history

        for i in 0..self.appointments.len() {
            if let Some(appointment) = self.appointments.get(i) {
                if appointment.doctor_id == doctor_id && !appointment.is_open {
                    history.push(appointment.clone()); // Add the closed appointment to the history
                }
            }
        }
    
        history 
    }

    // Get doctor medicine
    pub fn get_all_registered_medicines(&self) -> Vec<Medicine>{
        return self.drugs.iter().cloned().collect()
    }

    pub fn get_medicine_by_id(&self, medicine_id: u32) -> Option<Medicine> {
        // Assuming `self.medicines` is a collection (e.g., a vector or map) of medicines
        self.drugs.iter().find(|&medicine| medicine.id == medicine_id).cloned()
    }

   
}

    // pub fn check_user_exists(&self, pubkey: AccountId) -> bool {
    //     self.users.get(&pubkey).is_some();
    // }

    // // Create a new account
    // pub fn create_account(&mut self, name: String, user_type: String) {
    //     let caller = env::predecessor_account_id();
    //     assert!(!self.check_user_exists(caller.clone()), "User already exists");
    //     assert!(!name.is_empty(), "Username cannot be empty");

    //     let user = User {
    //         name: name.clone(),
    //         user_type,
    //         friend_list: Vector::new(b"f"),
    //     };

    //     self.users.insert(&caller, &user);
    //     self.all_users.push(&user);
    // }

    // // Get user information
    // pub fn get_username_type(&self, pubkey: AccountId) -> Option<User> {
    //     self.users.get(&pubkey);
    // }

    // // Add a friend
    // pub fn add_friend(&mut self, friend_key: AccountId, my_address: AccountId, name: String) {
    //     assert!(self.check_user_exists(my_address.clone()), "Create an account first");
    //     assert!(self.check_user_exists(friend_key.clone()), "User is not registered");
    //     assert!(my_address != friend_key, "Users cannot add themselves as friends");

    //     if !self.check_already_friends(my_address.clone(), friend_key.clone()) {
    //         self._add_friend(my_address.clone(), friend_key.clone(), name.clone());
    //         self._add_friend(friend_key, my_address, self.users.get(&my_address).unwrap().name.clone());
    //     }
    // }

    // // Check if users are already friends
    // fn check_already_friends(&self, pubkey1: AccountId, pubkey2: AccountId) -> bool {
    //     if let Some(user1) = self.users.get(&pubkey1) {
    //         for friend in user1.friend_list.iter() {
    //             if friend.pubkey == pubkey2 {
    //                 return true;
    //             }
    //         }
    //     }
    //     false
    // }

    // // Internal function to add a friend
    // fn _add_friend(&mut self, me: AccountId, friend_key: AccountId, name: String) {
    //     let mut user = self.users.get(&me).unwrap();
    //     let new_friend = Friend { pubkey: friend_key, name };
    //     user.friend_list.push(&new_friend);
    //     self.users.insert(&me, &user);
    // }

    // // Get friend list
    // pub fn get_my_friend_list(&self, _address: AccountId) -> Vec<Friend> {
    //     if let Some(user) = self.users.get(&_address) {
    //         return user.friend_list.to_vec();
    //     }
    //     vec![]
    // }

    // // Get chat code (hash of two public keys)
    // fn _get_chat_code(pubkey1: AccountId, pubkey2: AccountId) -> (AccountId, AccountId) {
    //     if pubkey1 < pubkey2 {
    //         (pubkey1, pubkey2)
    //     } else {
    //         (pubkey2, pubkey1)
    //     }
    // }

    // // Send message
    // pub fn send_message(&mut self, friend_key: AccountId, my_address: AccountId, msg: String) {
    //     assert!(self.check_user_exists(my_address.clone()), "Create an account first");
    //     assert!(self.check_user_exists(friend_key.clone()), "User is not registered");
    //     assert!(self.check_already_friends(my_address.clone(), friend_key.clone()), "You are not friend with the given user");

    //     let chat_code = self._get_chat_code(my_address.clone(), friend_key.clone());
    //     let new_msg = Message {
    //         sender: my_address.clone(),
    //         timestamp: env::block_timestamp(),
    //         content: msg.clone(),
    //     };

    //     let mut messages = self.all_messages.get(&chat_code).unwrap_or_else(|| Vector::new(b"msg"));
    //     messages.push(&new_msg);
    //     self.all_messages.insert(&chat_code, &messages);

    //     // Add notifications (pseudo-code, implement actual notification logic)
    //     // self.add_notification(my_address.clone(), "You have successfully sent a message", "Message".to_string());
    //     // self.add_notification(friend_key.clone(), "You have a new message", "Message".to_string());
    // }

    // // Read messages
    // pub fn get_read_message(&self, friend_key: AccountId, my_address: AccountId) -> Vec<Message> {
    //     let chat_code = self._get_chat_code(my_address, friend_key);
    //     self.all_messages.get(&chat_code).unwrap_or_else(Vec::new)
    // }

    // // Get all users
    // pub fn get_all_app_user(&self) -> Vec<User> {
    //     self.all_users.to_vec();
    // }


// Tests in a separated file (see more here -> http://xion.io/post/code/rust-unit-test-placement.html)
// #[cfg(test)]
// #[path = "./tests.rs"]
// mod tests;