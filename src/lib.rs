// Find all our documentation at https://docs.near.org
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::env::panic_str;
use near_sdk::{log, near};
use near_sdk::require;
use near_sdk::serde::{Serialize,Deserialize};
use near_sdk::AccountId;
use near_sdk::Balance;
use near_sdk::Promise;
use near_sdk::{env, near_bindgen};
use std::collections::{HashMap, HashSet};
use std::hash::BuildHasher;

pub mod internal;

#[near_bindgen]
#[derive(Serialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Address{
    address: String,
    country: String,
    state_or_province: String,
    city: String
}

#[near_bindgen]
#[derive(Serialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct PatientInput {
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

#[near_bindgen]
#[derive(Serialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Patient {
    id: u64,
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
    medical_history: Vec<String>,
    bought_medicine: Vec<i32>,
}

#[near_bindgen]
#[derive(Serialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
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
    college_address: Address,
    account_id: String,
    profile_pic: String,
    address: AccountId,
    bio: String,
}

#[near_bindgen]
#[derive(Serialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Doctor {
    id: u64,
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
    college_address: Address,
    account_id: String,
    profile_pic: String,
    address: AccountId,
    bio: String,
    appointment_counts: i32,
    successful_treaments: i32,
    is_approved: bool,
}

#[near_bindgen]
#[derive(Serialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Medicine {
    id: u64,
    doctor_id: u32,
    name: String,
    brand: String,
    manufacturer: String,
    manufacturing_date: String,
    expiry_date: String
    company_email: String,
    discount: i32,
    manufacturer_address: Address,
    price: u64,
    quantity: u64,
    current_location: Address,
    phone_no: u64,
    image: String,
    description: String,
    availability: bool,
}

#[near_bindgen]
#[derive(Serialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Prescription {
    id: u64,
    medicine_id: u64,
    patient_id: u64,
    doctor_id: u64,
    date: u64,
}

#[near_bindgen]
#[derive(Serialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Appointment {
    id: u64,
    patient_id: u64,
    doctor_id: u64,
    from: String,
    to: String,
    appointment_date: String,
    condition: String,
    message: String,
    is_open:bool,
}


//define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    owner: AccountId,
    admins: UnorderedSet<AccountId>,
    patients: Vector<Patient>,
    doctors: Vector<Doctor>,
    drugs: Vector<Medicine>,
    prescriptions: Vector<Prescription>,
    appointments: Vector<Appointment>,
    notifications: Vector<Notification>,
}

#[near_bindgen]
#[derive(Serialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Message{
    account_id: AccountId,
    timestamp: u64,
    message: String,
}

#[near_bindgen]
#[derive(Serialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Friends {
    account_id: AccountId,
}

#[near_bindgen]
#[derive(Serialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Order {
     medicine_id: u64,
     price: u64,
     payment_amount: u64,
     quantity: i32,
     patient_id: u64,
     date: u64,
}

#[near_bindgen]
#[derive(Serialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Notification {
    id: u64,
    account_id: AccountId,
    message: String,
    timestamp: u64,
    category: String,
}

//Default state to use if no initialize methods called
//Define the default, which automatically initializesthe contract
impl Default for Contract{
    fn default() -> Self {
        owner: "owner_testnet_url".parse().unwrap(),
        admins: UnorderedSet::new(b"s"),
        patients: Vector::new(b"a"),
        doctors: Vector::new(b"b"),
        drugs: Vector::new(b"c"),
        prescriptions: Vector::new(b"d"),
        appointments: Vector::new(b"e"),
        notifications: Vector::new(b"k")
    }
}

//Implement the contract structure
#[near_bindgen]
impl Contract {
    #[init]
    pub fn init(owner: AccountId,admins: Option<Vec<AccountId>> ) -> Self {
        assert!(!env::state_exists(),"Already initialized");

        Self{
            owner,
            admins: account_vec_to_set(
                if admins.is_some() {
                    admins.unwrap()
                } else {
                    vec![]
                },
                b"s",
            )
            doctors: Vector::new();
            appointment_counter: 0,
            prescription_counter: 0,
        }
    }

    pub fn add_notification(&mut self, user_address: AccountId, message: String, category_type: String) {
        let timestamp = env::block_timestamp();

        let mut user_notifications = self.notifications.get(&user_address).unwrap_or_else(Vec::new);

        let new_notification = Notification {
            id: user_notifications.len() as u64,
            user_address: user_address.clone(),
            message,
            timestamp,
            category_type,
        };

        user_notifications.push(new_notification);

        self.notifications.insert(&user_address, &user_notifications);

        env::log_str(&format!("Notification sent to {} at {}", user_address, timestamp));
    }


    pub fn get_notifications(&self) -> Vec<Notification>{
        return self.notifications.to_vec();
    }

    pub fn add_medicine(
        &self mut,
        id: u64,
        doctor: &doctor_id,
        name: String,
        brand: String,
        manufacturer: String,
        manufacturing_date: u64,
        expiry_date: u64,
        company_email: String,
        discount: i32,
        manufacturer_address: String,
        price: i32,
        quantity: i32,
        current_location: Address,
        phone_no: u64,
        image: String,
        description: String,
    ) {
        assert!(
            self.is_admin(),
            "Only the  admins can call this method"
        );
        let medicine = Medicine {
            id: 
            ipfs_url,
            price,
            quantity,
            discount,
            location,
            availability
        };

        self.drugs.insert(medicine);
    }

    //========== End of Medicine =======

    //========== Doctor =========----
    pub fn add_doctor(
        &mut self,
        id: u64,
        doctor: DoctorInput
    ) -> Self{
        !require(
            !self.doctors.contains_key(id),
            "Doctor with this ID Already exists!"
        );


        self doctor = Doctor {
            id: id.clone(),
            title:doctor.title,
            first_name: doctor.first_name,
            last_name: doctor.last_name
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

        self.doctors.insert(id.clone(),doctor);
    }

    pub fn approve_doctor(&mut self, doctor_id: u64) -> bool {
         //only the admin is required to invoke this function
        if let Some(mut doctor) = self.doctors.get(&doctor_id) {
            doctor.is_approved = true;
            self.doctors.insert(&doctor_id, &doctor);
            true
        } else {
            false
        }
    }

    //Update by the doctor
    pub fn update_patient_medical(&self mut,patient_id: u64,new_medical_history: String) -> Self{
        //only the doctor is required to invoke this function
        //get patient by id and push new medical history
        //Insert 
    }

    pub fn complete_appointment(&mut self, appointment_id: u64) {
        let mut appointment = self.appointments.get(&appointment_id).expect("Appointment does not exist");
        let doctor_id = self.get_doctor_id(env::predecessor_account_id());
        assert_eq!(appointment.doctor_id, doctor_id, "Only the assigned doctor can complete the appointment");

        appointment.is_open = false;
        self.appointments.insert(&appointment_id, &appointment);

        let mut doctor = self.doctors.get(&env::predecessor_account_id()).unwrap();
        doctor.successful_treatment_count += 1;
        self.doctors.insert(&env::predecessor_account_id(), &doctor);

        self.add_notification(env::predecessor_account_id(), "You have successfully completed the appointment".to_string(), "Doctor".to_string());
        self.add_notification(self.get_patient_account_id(appointment.patient_id), "Your Appointment is successfully completed".to_string(), "Patient".to_string());
        self.add_notification(self.admin.clone(), "Doctor completed appointment successfully".to_string(), "Admin".to_string());
    }

   
    pub fn prescribe_medicine(&mut self, medicine_id: u64, patient_id: u64) -> u64 {
        let doctor_id = self.get_doctor_id(env::predecessor_account_id());
        assert!(self.doctors.get(&env::predecessor_account_id()).unwrap().is_approved, "Doctor is not approved");

        let prescription_id = self.prescription_counter;
        self.prescription_counter += 1;

        let prescription = Prescription {
            id: prescription_id,
            medicine_id,
            patient_id,
            doctor_id,
            date: env::block_timestamp(),
        };

        self.prescriptions.insert(&prescription_id, &prescription);

        self.add_notification(env::predecessor_account_id(), "You have successfully prescribed medicine".to_string(), "Doctor".to_string());
        self.add_notification(self.get_patient_account_id(patient_id), "Doctor prescribed you medicine".to_string(), "Patient".to_string());
        self.add_notification(self.admin.clone(), "Doctor prescribed medicine successfully".to_string(), "Admin".to_string());

        prescription_id
    }

    //======== End Of Doctor
    //===========  Patient
    pub fn add_patient(&mut self,patient: PatientInput) -> {
        require!(
            !self.patients.contains_key(id),
            "This user already exists in the system"
        );

        let patient = Patient {
            id: id.clone(),
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
            message: patient.message
            medical_history: patient.condition,
            bought_medicine: vec![],
        };

        self.patients.insert(id.clone(),patient);

        log!("Patient was registered successfully!")
    }

    pub fn book_appointment(&mut self, doctor_id: u64, from: String, to: String, appointment_date: String, condition: String, message: String) -> u64 {
        let patient_id = self.get_patient_id(env::predecessor_account_id());
        let appointment_id = self.appointment_counter;
        self.appointment_counter += 1;

        let appointment = Appointment {
            id: appointment_id,
            patient_id,
            doctor_id,
            from,
            to,
            appointment_date,
            condition,
            message,
            is_open: true,
        };

        self.appointments.insert(&appointment_id, &appointment);
        
        self.add_notification(env::predecessor_account_id(), "You have successfully booked an appointment".to_string(), "Patient".to_string());
        self.add_notification(self.get_doctor_account_id(doctor_id), "A new appointment has been booked for you".to_string(), "Doctor".to_string());
        self.add_notification(self.admin.clone(), "A new appointment has been booked".to_string(), "Admin".to_string());

        appointment_id
    }

    pub fn count_available_medicines(&self) -> u64 {
        self.medicines.values().filter(|m| m.availability && m.quantity > 0).count() as u64
    }

    pub fn is_in_stock(&self, medicine_id: u64) -> bool {
        if let Some(medicine) = self.medicines.get(&medicine_id) {
            medicine.availability && medicine.quantity > 0
        } else {
            false
        }
    }

    pub fn buy_medicine(&mut self, medicine_id: u64, quantity: i32, patient_id: u64) -> Promise {
        let mut medicine = self.medicines.get(&medicine_id).expect("Medicine not found");
        assert!(medicine.availability, "Medicine is not available");
        assert!(medicine.quantity >= quantity as u64, "Not enough stock");

        let discounted_price = medicine.price * (100 - medicine.discount as u64) / 100;
        let total_price = discounted_price * quantity as u64;

        let order = Order {
            medicine_id,
            price: discounted_price,
            payment_amount: total_price,
            quantity,
            patient_id,
            date: env::block_timestamp(),
        };

        medicine.quantity -= quantity as u64;
        if medicine.quantity == 0 {
            medicine.availability = false;
        }

        self.medicines.insert(&medicine_id, &medicine);
        
        let order_id = self.order_counter;
        self.order_counter += 1;
        self.orders.insert(&order_id, &order);

        // Transfer the payment to the contract
        Promise::new(env::current_account_id()).transfer(total_price)
    }

    // End of patient
    // Admin

    //Update by Admin only
    pub fn update_registration_fee() -> Self{

    }

    pub fn update_appointment_fee() -> Self{

    }

    pub fn update_patient_registration_fee() -> Self{

    }

    pub fn update_admin_address() -> Self{

    }

    //======== End Of Admin

    //=========  Get patient data
    pub fn get_all_patient_orders() -> Self{

    }

    pub fn get_all_prescription_details() -> Self{

    }

    pub fn get_all_prescribed_medicines() -> Self{

    }

    pub fn get_all_prescribed_medicine_of_patient() -> Self{

    }

    pub fn get_all_registered_patients( &self) -> Vec<Patient>{
        return self.patients.to_vec();
    }

    pub fn get_patient_id(
        &self
    ) -> Option<Patient>{
        if let Some(index) = self.patients.iter().position(|patient| patient.id == id) {
            self.patients.get(index as u64);
        } else {
            None;
        }
    }

    pub fn get_patient_appointment(&self) -> Vec<Appointment>{
        let appointments = self
            .appointments
            .values() 
            .filter(|appointment| {
                appointment.patient_id == patient_id.clone()
            }) 
            .cloned() 
            .collect();

        appointments
    }

    pub fn get_patient_medical_history(&self) -> Vec<Patient>{
        !require(
            !self.patients.contains_key(id),
            "Doctor with this ID Already exists!"
        );
        assert!(
            self.is_owner_or_admin(),
            "Only the owner(patient) and admins can call this method"
        );
        return self.patients.medical_history;
    }

    pub fn get_patient_appointment_history() -> Self{

    }

    pub fn get_patient_details() -> Self{

    }

    pub fn get_bought_medicine_by_patient() -> Self{

    }

    pub fn get_all_appointments(&self) -> Vec<Appointment>{
        return self.appointments.to_vec();
    }

    // Get doctors data
    pub fn get_all_doctors_data(&self) -> Vec<Doctor>{
        return self.doctors.to_vec();
    }

    pub fn get_approved_doctors(&self) -> Vec<Doctor> {
        self.doctors.values().filter(|doctor| doctor.is_approved).collect()
    }


    pub fn get_most_popular_doctor() -> Vec<Doctor>{
        let mut most_popular: Vec<Doctor> = Vec::new();
        let mut highest_popularity = 0;

        // Iterate through the vector of doctors
        for doctor in self.doctors.iter() {
            // Calculate the popularity of the current doctor
            let popularity = doctor.appointment_counts + doctor.successful_treatments;

            // If the current doctor is more popular, reset the list
            if popularity > highest_popularity {
                highest_popularity = popularity;
                most_popular.clear(); // Clear the previous results
                most_popular.push(doctor); // Add the new most popular doctor
            } else if popularity == highest_popularity {
                // If there's a tie, add the doctor to the list
                most_popular.push(doctor);
            }
        }
    }

    pub fn get_doctor_details() -> Self{

    }

    pub fn get_doctor_id(
        &self,
        id: u64,
    ) -> Option<Doctor>{
        if let Some(index) = self.doctors.iter().position(|doctor| doctor.id == id) {
            self.doctors.get(index as u64)
        } else {
            None
        }
    }

    pub fn get_doctor_appointment_historys(&self, doctor_id: u64) -> Vec<Appointment>{
        let mut doctor_appointments: Vec<Appointment> = Vec::new();

        // Iterate through the list of appointments
        for appointment in self.appointments.iter() {
            // Check if the appointment belongs to the specified doctor
            if appointment.doctor_id == doctor_id {
                doctor_appointments.push(appointment);
            }
        }
    }

    pub fn get_doctor_appointments(&self, doctor_id: u64) -> Vec<Appointment> {
        self.appointments.values()
            .filter(|appointment| appointment.doctor_id == doctor_id)
            .collect()
    }

    // Get doctor medicine
    pub fn get_all_registered_medicines() -> Vec<Medicine>{
        return self.drugs.to_vec()
    }

    pub fn get_medicine_details() -> Self {

    }


}