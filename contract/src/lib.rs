use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen};
use near_sdk::serde::{Serialize, Deserialize};

// Define the contract structure
#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize, Clone, Serialize, Deserialize)]
pub struct Identity {
    first_name: String,
    last_name: String,
    dni: String,
    email: String,
    country: String,
    city: String,
    age: u8,
    date_of_birth: String,
}

// Implement the contract structure
#[near_bindgen]
impl Identity {
    // Public constructor - this will be used to create new identity
    #[init]
    pub fn new() -> Self {
        Self {
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            dni: "1545218".to_string(),
            email: "john.doe@gmail.com".to_string(),
            country: "USA".to_string(),
            city: "New York".to_string(),
            age: 30,
            date_of_birth: "1990-01-01".to_string(),
        }
    }

    // Public method
    pub fn get_identity(&self) -> Identity {
        return self.clone();
    }
}


#[cfg(test)]
mod tests {

    #[test]
    pub fn test_get_identity() {
        let identity = super::Identity::new();
        assert_eq!(identity.first_name, "John");
        assert_eq!(identity.last_name, "Doe");
        assert_eq!(identity.dni,     "1545218");
        assert_eq!(identity.email, "john.doe@gmail.com");
        assert_eq!(identity.country, "USA");
        assert_eq!(identity.city,  "New York");
        assert_eq!(identity.age, 30);
        assert_eq!(identity.date_of_birth, "1990-01-01");
    }
}

