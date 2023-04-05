use std::env;
use std::fs;
use std::str;
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use rand::Rng;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let mut vault = Vault::struct_from_file("Test.json");

    vault.sort_credentials();
    
    vault.to_file("Test.json");
}

#[derive(Serialize, Deserialize, Debug)]
struct Vault{
    salt: u128,
    nonce: u128,
    encrypted: bool,
    credentials: Vec<Credential>,
    notes: Vec<Note>,
    version: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Credential{
    id: String,
    name: String,
    favorite: bool,
    username: String,
    password: String,
    url: String,
}

impl Credential{
    fn new(name: &str, username: &str, password: &str, url: &str) -> Self{
        return Credential{
            id: Uuid::new_v4().to_string(),
            name: name.to_string(),
            favorite: false,
            username: username.to_string(),
            password: password.to_string(),
            url: url.to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Note{
    id: String,
    name: String,
    favorite: bool,
    content: String,
}

impl Note{
    fn new(name: &str, content: &str) -> Self{
        return Note{
            id: Uuid::new_v4().to_string(),
            name: name.to_string(),
            favorite: false,
            content: content.to_string(),
        }
    }
}

impl Vault{
    fn new() -> Self{
        return Vault{
            salt: 0,
            nonce: 0,
            encrypted: false,
            credentials: vec![],
            notes: vec![],
            version: VERSION.to_string(),
        }
    }

    fn struct_from_file(s: &str) -> Self{
        let path = env::current_dir().unwrap();
        let path = path.join(s);

        let contents = fs::read_to_string(&path).expect("Something went wrong reading the file");
    
        let vault: Vault = serde_json::from_str(&contents).unwrap();
        return vault
    }

    fn to_file(&self, s: &str){
        let path = env::current_dir().unwrap();
        let path = path.join(s);
    
        let serialized = serde_json::to_string(self).unwrap();
        fs::write(&path, &serialized).expect("Unable to write file");
    }

    fn add_credential(&mut self, credential: Credential){
        self.credentials.push(credential);
    }

    fn add_note(&mut self, note: Note){
        self.notes.push(note);
    }

    fn sort_credentials(&mut self){
        self.credentials.sort_by(|a, b| {
            if a.favorite && !b.favorite{
                return std::cmp::Ordering::Less;
            }
            else if !a.favorite && b.favorite{
                return std::cmp::Ordering::Greater;
            }
            else{
                return a.name.cmp(&b.name);
            }
        });
    }

    fn sort_notes(&mut self){
        self.notes.sort_by(|a, b| {
            if a.favorite && !b.favorite{
                return std::cmp::Ordering::Less;
            }
            else if !a.favorite && b.favorite{
                return std::cmp::Ordering::Greater;
            }
            else{
                return a.name.cmp(&b.name);
            }
        });
    }
    
    fn find_credential(&self, id: &str) -> Option<usize>{
        for (index, credential) in self.credentials.iter().enumerate(){
            if credential.id == id{
                return Some(index);
            }
        }
        return None;
    }

    fn find_note(&self, id: &str) -> Option<usize>{
        for (index, note) in self.notes.iter().enumerate(){
            if note.id == id{
                return Some(index);
            }
        }
        return None;
    }

    fn remove_credential_by_id(&mut self, id: &str) -> Result<(), String>{
        match self.find_credential(id){
            Some(index) => {
                self.credentials.remove(index);
                return Ok(());
            },
            None => {
                return Err("Credential not found".to_string());
            }
        }
    }
    
    fn remove_note_by_id(&mut self, id: &str) -> Result<(), String>{
        match self.find_note(id){
            Some(index) => {
                self.notes.remove(index);
                return Ok(());
            },
            None => {
                return Err("Note not found".to_string());
            }
        }
    }
}
