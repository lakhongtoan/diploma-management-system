#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype,
    Address, Env, String,
};

#[contracttype]
#[derive(Clone)]
pub struct Diploma {
    pub diploma_id: String,
    pub student_name: String,
    pub student_id: String,
    pub major: String,
    pub grade: String,
    pub issue_date: String,
    pub file_hash: String,
    pub issuer: Address,
}

#[contracttype]
pub enum DataKey {
    Diploma(String),
}

#[contract]
pub struct DiplomaContract;

#[contractimpl]
impl DiplomaContract {

    // Thêm bằng tốt nghiệp
    pub fn add_diploma(
        env: Env,
        diploma_id: String,
        student_name: String,
        student_id: String,
        major: String,
        grade: String,
        issue_date: String,
        file_hash: String,
        issuer: Address,
    ) {
        let key = DataKey::Diploma(diploma_id.clone());

        if env.storage().persistent().has(&key) {
            panic!("Diploma already exists");
        }

        let diploma = Diploma {
            diploma_id,
            student_name,
            student_id,
            major,
            grade,
            issue_date,
            file_hash,
            issuer,
        };

        env.storage().persistent().set(&key, &diploma);
    }

    // Tra cứu bằng
    pub fn get_diploma(
        env: Env,
        diploma_id: String,
    ) -> Option<Diploma> {
        let key = DataKey::Diploma(diploma_id);

        env.storage()
            .persistent()
            .get::<DataKey, Diploma>(&key)
    }

    // Kiểm tra tồn tại
    pub fn diploma_exists(
        env: Env,
        diploma_id: String,
    ) -> bool {
        let key = DataKey::Diploma(diploma_id);

        env.storage()
            .persistent()
            .has(&key)
    }

    // Xóa bằng
    pub fn delete_diploma(
        env: Env,
        diploma_id: String,
    ) {
        let key = DataKey::Diploma(diploma_id);

        env.storage()
            .persistent()
            .remove(&key);
    }
}