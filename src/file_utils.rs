use std::{fs, io, path::PathBuf};

pub fn read_key_from_file(key_path: &PathBuf) -> io::Result<Vec<u8>> {
    let key = fs::read(key_path)?;
    if key.len() != 32 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!(
                "Niepoprawna długość klucza: oczekiwano 32 bajtów, otrzymano {}",
                key.len()
            ),
        ));
    }
    Ok(key)
}

pub fn write_plain_file(file_path: &PathBuf, plaintext: &[u8]) -> io::Result<()> {
    fs::write(file_path, plaintext)?;
    Ok(())
}

pub fn read_plain_file(file_path: &PathBuf) -> io::Result<Vec<u8>> {
    let plaintext = fs::read(file_path)?;
    Ok(plaintext)
}

pub fn write_encrypted_file(
    file_path: &PathBuf,
    nonce: &[u8],
    ciphertext: &[u8],
    salt_opt: &Option<[u8; 16]>,
) -> io::Result<()> {
    let mut data: Vec<u8> = Vec::new();
    if let Some(salt) = salt_opt {
        data.extend_from_slice(b"PSWD");
        data.extend_from_slice(salt);
    } else {
        data.extend_from_slice(b"KEY0");
    };
    data.extend_from_slice(nonce);
    data.extend_from_slice(ciphertext);
    fs::write(file_path, data)?;
    Ok(())
}

pub fn read_encrypted_file(file_path: &PathBuf) -> io::Result<(Vec<u8>, Vec<u8>, Option<Vec<u8>>)> {
    let data: Vec<u8> = fs::read(file_path)?;
    let header = &data[..4];
    let (salt, offset) = match header {
        b"PSWD" => {
            if data.len() < 4 + 12 + 16 {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Plik zaszyfrowany hasłem za krótki, brak nonce lub soli",
                ));
            }
            let salt = data[4..20].to_vec();
            (Some(salt), 20)
        }
        b"KEY0" => {
            if data.len() < 4 + 12 {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Plik zaszyfrowany kluczem za krótki, brak nonce",
                ));
            }
            (None, 4)
        }
        _ => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Nieznany nagłówek pliku",
            ));
        }
    };
    let nonce = data[offset..offset + 12].to_vec();
    let ciphertext = data[offset + 12..].to_vec();
    Ok((nonce, ciphertext, salt))
}

pub fn get_decrypted_filename(encrypted: &PathBuf) -> PathBuf {
    let mut new_path = encrypted.clone();
    if let Some(ext) = new_path.extension() {
        if ext == "enc" {
            new_path.set_extension("");
        }
    }
    new_path
}

pub fn get_encrypted_filename(original: &PathBuf) -> PathBuf {
    let mut new_path = original.clone();
    if let Some(ext) = new_path.extension() {
        let mut ext_osstring = ext.to_os_string();
        ext_osstring.push(".enc");
        new_path.set_extension(ext_osstring);
    } else {
        new_path.set_extension("enc");
    }
    new_path
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_name_generation() {
        let path = PathBuf::from("test.txt");
        let encrypted = get_encrypted_filename(&path);
        assert_eq!(encrypted, PathBuf::from("test.txt.enc"));

        let decrypted = get_decrypted_filename(&encrypted);
        assert_eq!(decrypted, PathBuf::from("test.txt"));
    }
}
