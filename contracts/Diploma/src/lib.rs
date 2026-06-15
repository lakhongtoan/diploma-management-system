
#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, String, Symbol, Vec};

// Struktur data yang akan menyimpan notes
#[contracttype]
#[derive(Clone, Debug)]
pub struct Note {
    id: u64,
    title: String,#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype,
    Address, Env, String, Symbol,
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
    content: String,
}

// Storage key untuk data notes
const NOTE_DATA: Symbol = symbol_short!("NOTE_DATA");

#[contract]
pub struct NotesContract;

#[contractimpl]
impl NotesContract {
    pub fn get_notes(env: Env) -> Vec<Note> {
        // 1. ambil data notes dari storage
        return env.storage().instance().get(&NOTE_DATA).unwrap_or(Vec::new(&env));
    }

    // Fungsi untuk membuat note baru
    pub fn create_note(env: Env, title: String, content: String) -> String {
        // 1. ambil data notes dari storage
        let mut notes: Vec<Note> = env.storage().instance().get(&NOTE_DATA).unwrap_or(Vec::new(&env));
        
        // 2. Buat object note baru
        let note = Note {
            id: env.prng().gen::<u64>(),
            title: title,
            content: content,
        };
        
        // 3. tambahkan note baru ke notes lama
        notes.push_back(note);
        
        // 4. simpan notes ke storage
        env.storage().instance().set(&NOTE_DATA, &notes);
        
        return String::from_str(&env, "Notes berhasil ditambahkan");
    }

    // Fungsi untuk menghapus notes berdasarkan id
    pub fn delete_note(env: Env, id: u64) -> String {
        // 1. ambil data notes dari storage 
        let mut notes: Vec<Note> = env.storage().instance().get(&NOTE_DATA).unwrap_or(Vec::new(&env));

        // 2. cari index note yang akan dihapus menggunakan perulangan
        for i in 0..notes.len() {
            if notes.get(i).unwrap().id == id {
                notes.remove(i);

                env.storage().instance().set(&NOTE_DATA, &notes);
                return String::from_str(&env, "Berhasil hapus notes");
            }
        }

        return String::from_str(&env, "Notes tidak ditemukan")
    }
}

mod test;