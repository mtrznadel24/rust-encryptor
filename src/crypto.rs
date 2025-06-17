use std::{fs, io, path::PathBuf};

use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng, Error},
    Aes256Gcm, Nonce, Key // Or `Aes128Gcm`
};
 

pub fn generate_key(output: &PathBuf) -> io::Result<()> {
    let key = Aes256Gcm::generate_key(OsRng);
    fs::write(output, key.as_slice())?;
    Ok(())

}

pub fn encrypt(key: Vec<u8>, plaintext: Vec<u8>) -> Result<(Vec<u8>, Vec<u8>), Error> {
    let key = Key::<Aes256Gcm>::from_slice(key.as_slice());
    let cipher= Aes256Gcm::new(&key);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    let ciphertext = cipher.encrypt(&nonce, plaintext.as_slice())?;
    Ok((nonce.to_vec(), ciphertext))
}

pub fn decrypt(key: Vec<u8>, nonce: Vec<u8>, ciphertext: Vec<u8>) -> Result<Vec<u8>, Error> {
    let key = Key::<Aes256Gcm>::from_slice(key.as_slice());
    let cipher= Aes256Gcm::new(&key);
    let nonce = nonce.as_slice();
    let nonce = Nonce::from_slice(nonce);
    let plaintext = cipher.decrypt(&nonce, ciphertext.as_slice())?;
    Ok(plaintext)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encrypt_decrypt() {
        let key = Aes256Gcm::generate_key(OsRng);
        let plaintext = b"Hello world!".to_vec();
        let (nonce, ciphertext) = encrypt(key.to_vec(), plaintext.clone()).unwrap();
        let decrypted = decrypt(key.to_vec(), nonce, ciphertext).unwrap();
        assert_eq!(plaintext, decrypted);
    }

}