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
    date: u64,
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
    }
}

//Implement the contract structure
#[near_bindgen]
impl Contract {
    #[init]
    pub fn init(
        owner: AccountId,
        admins: Option<Vec<AccountId>>
    ) -> Self {
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
        }
    }

    pub fn add_notification(
        &mut self,

    ) -> Self{
 
    }

    pub fn get_notifications(&self) -> Vec{

    }

    pub fn add_medicine(
        &self mut,
        id: u64,
    ) {
        assert!(
            self.is_admin(),
            "Only the  admins can call this method"
        );
        let medicine = Medicine {
            id,
            ipfs_url,
            price,
            quantity,
            discount,
            location,
            availability
        };

        self.drugs.insert(medicine),
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

    pub fn approve_doctor() -> Self{

    }

    //Update by the doctor
    pub fn update_patient_medical() -> Self{

    }

    pub fn complete_appointment() -> Self{

    }

    pub fn prescribe_medicine() -> Self{

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

    pub fn book_appointment() -> Self {

    }

    pub fn buy_medicine() -> Self{

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

    pub fn get_patient_appointment() -> Vec<Appointment>{
        //by patient id and appointment Id
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

    pub fn get_all_approved_doctors(&self) -> Vec<Doctor>{

    }

    pub fn get_most_popular_doctor(

    ) -> Self{

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

    pub fn get_doctor_appointment_historys() -> Self{

    }

    // Get doctor medicine
    pub fn get_all_registered_medicines() -> Vec<Medicine>{
        return self.drugs.to_vec()
    }

    pub fn get_medicine_details() -> Self {

    }


}