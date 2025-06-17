use zxcvbn::zxcvbn;
use argon2::{
    password_hash::{
        rand_core::OsRng,
        SaltString, Error
    },
    Argon2
};


pub fn generate_salt() -> [u8; 16] {
    let salt = SaltString::generate(&mut OsRng);
    let salt = salt.as_str().as_bytes();
    let mut salt_array = [0u8; 16];
    salt_array.copy_from_slice(&salt[..16]);
    salt_array
}

pub fn create_key_from_password(password: &str, salt: &[u8; 16]) -> Result<Vec<u8>, Error>{
    let mut key = [0u8; 32];
    Argon2::default().hash_password_into(password.as_bytes(), salt, &mut key)?;
    Ok(key.to_vec())
}

pub fn is_password_safe(password: &str) -> bool {
    match zxcvbn(password, &[]) {
        Ok(estimate) => {
            if estimate.score() >= 3 {
                println!("Hasło jest wystarczająco silne. Siła hasła: {}", estimate.score());
                true
            }
            else {
                println!("Hasło jest zbyt słabe. Siła hasła: {}", estimate.score());
                false
            }
        }
        Err(e) => {
            eprintln!("Wystąpił błąd podczas oceny hasła. {}", e);
            false
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_salt_length() {
        let salt = generate_salt();
        assert_eq!(salt.len(), 16);
    }

    #[test]
    fn key_from_password_length() {
        let salt = generate_salt();
        let key = create_key_from_password("ExamplePassword123!", &salt).unwrap();
        assert_eq!(key.len(), 32);
    }

    #[test]
    fn test_is_password_safe() {
        let weak_pswd = "123";
        assert!(!is_password_safe(weak_pswd));
        let strong_password = "H5kF!4^fsl#G";
        assert!(is_password_safe(strong_password));
    }
}