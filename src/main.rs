use std::env;
use std::fs;
use std::str;
use uuid::Uuid;
use serde::{Serialize, Deserialize};

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let mut vault = Vault::new();

    vault.add_credential(Credential::new("Steam", "Eldoritto", "123321852@", "https://store.steampowered.com/"));
    vault.add_credential(Credential::new("Epic", "BB-8", "123211312@", "https://store.epicgames.com/en-US/"));
    vault.add_note(Note::new("Test", "This is a test note"));

    vault.credentials[1].name = "Epic Games".to_string();
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
    username: String,
    password: String,
    url: String,
}

impl Credential{
    fn new(name: &str, username: &str, password: &str, url: &str) -> Self{
        return Credential{
            id: Uuid::new_v4().to_string(),
            name: name.to_string(),
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
    content: String,
}

impl Note{
    fn new(name: &str, content: &str) -> Self{
        return Note{
            id: Uuid::new_v4().to_string(),
            name: name.to_string(),
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
}
