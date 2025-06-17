mod cli;
mod crypto;
mod file_utils;
mod password;

use std::process::exit;
use std::io;
use cli::{Cli, Commands};
use clap::Parser;
use crypto::{generate_key, encrypt, decrypt};
use file_utils::{read_key_from_file, read_plain_file, write_encrypted_file, get_encrypted_filename, read_encrypted_file, get_decrypted_filename, write_plain_file};
use password::{generate_salt, create_key_from_password, is_password_safe};

fn main() {
    let cli = Cli::parse();


    match &cli.command {
        Commands::Encrypt {args} => {

            if !args.file.exists() {
                eprintln!("Plik {:?} nie istnieje.", args.file);
                exit(1);
            }

            if let Some(password) = &args.password {
                if !is_password_safe(&password) {
                    println!("Czy chcesz kontynuować ? (y/n)");
                    let mut buffer = String::new();
                    loop {
                        buffer.clear();
                        io::stdin().read_line(&mut buffer).expect("Nie udało się odczytać wejścia");
                        match buffer.trim() {
                            "y" | "Y" => break,
                            "n" | "N" => exit(0),
                            _ => { 
                                println!("Wpisz 'n' lub 'y'");
                                continue; }
                        }
                    }
                          
                }    
            }

            let plaintext = read_plain_file(&args.file).unwrap_or_else(|e| {
                eprintln!("Nie udało się odczytać pliku: {}", e);
                exit(1);
            });
            let salt_opt: Option<[u8;16]>;
            
            let key: Vec<u8> = if let Some(key_path) = &args.key {
                salt_opt = None;
                read_key_from_file(key_path).unwrap_or_else(|e| {
                    eprintln!("Błąd odczytu klucza {}", e);
                    exit(1);
                })

            } else if let Some(password) = &args.password{
                let salt = generate_salt();
                salt_opt = Some(salt);
                create_key_from_password(password, &salt).unwrap_or_else(|e| {
                    eprintln!("Błąd tworzenia klucza z hasła {}", e);
                    exit(1);
                })

            } else {
                eprintln!("Nie podano pliku  kluczem lub hasła.");
                exit(1);
            };

            
            let (nonce, ciphertext) = encrypt(key, plaintext).unwrap_or_else(|e| {
                eprintln!("Błąd szyfrowania: {}", e);
                exit(1);
            });
            let new_path = get_encrypted_filename(&args.file);
            if let Err(e) = write_encrypted_file(&new_path, &nonce, &ciphertext, &salt_opt){
                eprintln!("Błąd zapisu zaszyfrowanego pliku {}", e);
                exit(1);
            } else {
                println!("Zaszyfrowany plik zapisany do {:?}", &new_path)
            }
            if args.remove_original {
                std::fs::remove_file(&args.file).unwrap_or_else(|e| {
                    eprintln!("Nie udało sie usunąć orginalnego pliku: {}", e);
                    exit(1);
                })
            }
        }
        Commands::Decrypt { args } => {

            if !args.file.exists() {
                eprintln!("Plik {:?} nie istnieje.", args.file);
                exit(1);
            }

            let (nonce, ciphertext, salt_opt) = read_encrypted_file(&args.file).unwrap_or_else(|e| {
                eprintln!("Nie udało się odczytać pliku: {}", e);
                exit(1);
            });
            
            let key: Vec<u8> = if let Some(key_path) = &args.key {
                read_key_from_file(key_path).unwrap_or_else(|e| {
                    eprintln!("Błąd odczytu klucza {}", e);
                    exit(1);
                })

            } else if let Some(password) = &args.password{
                let salt: [u8; 16] = salt_opt.expect("Brak soli w pliku zaszyfrowanym hasłem").try_into().unwrap_or_else(|_| {
                    eprintln!("Nieprawidłowa długość soli — oczekiwano 16 bajtów.");
                    exit(1);
                });
                create_key_from_password(password, &salt).unwrap_or_else(|e| {
                    eprintln!("Błąd tworzenia klucza z hasła {}", e);
                    exit(1);
                })
            } else {
                eprintln!("Nie podano pliku  kluczem lub hasła.");
                exit(1);
            };

            let plaintext = decrypt(key, nonce, ciphertext).unwrap_or_else(|e| {
                eprintln!("Błąd odszyfrowania: {}", e);
                exit(1);
            });
            let new_path = get_decrypted_filename(&args.file);
            if let Err(e) = write_plain_file(&new_path, &plaintext){
                eprintln!("Błąd zapisu odszyfrowanego pliku {}", e);
                exit(1);
            } else {
                println!("Odszyfrowany plik zapisany do {:?}", &new_path)
            }
            if args.remove_original {
                std::fs::remove_file(&args.file).unwrap_or_else(|e| {
                    eprintln!("Nie udało sie usunąć orginalnego pliku: {}", e);
                    exit(1);
                })
            }
        }
        Commands::GenerateKey { output } => {
            if let Err(e) = generate_key(&output){
                eprintln!("Bład zapisu {}", e);
                std::process::exit(1);
            } else {
                println!("Klucz zapisany do pliku {:?}", &output)
            }

        }
    }
}   