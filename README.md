# 🔐 Rust Encryptor

A simple and secure command-line tool for encrypting and decrypting files using AES-256-GCM encryption. It supports both password-based and key-based encryption, with a strong emphasis on usability and safety.

---

## 📦 Features

- 🔑 Encrypt/decrypt files using AES-256-GCM  
- 🔐 Supports both key files and password-based encryption (with Argon2 key derivation)  
- 🧂 Automatically adds and reads salt when using passwords  
- 🧠 Password strength checking (via zxcvbn)  
- 🗑️ Option to remove the original file after encryption/decryption  
- 🧰 Built using `clap` for a clean command-line interface  

---

## ⚙️ Installation

Clone the repository and build using Cargo:

```bash
git clone https://github.com/YOUR_USERNAME/rust-encryptor.git
cd rust-encryptor
cargo build --release
```

---

## 🚀 Usage

```bash
rust-encryptor <COMMAND> [OPTIONS]
```

### 📜 Available Commands

| Command        | Description                         |
|----------------|-------------------------------------|
| `encrypt`      | Encrypts the specified file         |
| `decrypt`      | Decrypts the specified file         |
| `generate-key` | Generates a random 256-bit key      |
| `help`         | Shows help message                  |

---

### 🔐 Encrypt a file

With password:

```bash
rust-encryptor encrypt --file secret.txt --password "StrongPassword123!"
```

Or using a key:

```bash
rust-encryptor encrypt --file secret.txt --key path/to/keyfile
```

Add `--remove-original` to delete the original file after encryption.

---

### 🔓 Decrypt a file

With password:

```bash
rust-encryptor decrypt --file secret.txt.enc --password "StrongPassword123!"
```

With key file:

```bash
rust-encryptor decrypt --file secret.txt.enc --key path/to/keyfile
```

---

### 🧬 Generate a key

```bash
rust-encryptor generate-key --output keyfile.key
```

---

## 🛠️ Options

Global options used with `encrypt` and `decrypt`:

| Flag/Option         | Description                                  |
|---------------------|----------------------------------------------|
| `--file`, `-f`      | Path to the file to encrypt/decrypt          |
| `--password`, `-p`  | Password to derive the encryption key        |
| `--key`, `-k`       | Path to a file containing a 256-bit key      |
| `--remove-original` | Deletes the original file after operation    |

> ⚠️ Either `--password` or `--key` must be provided.

---

## 📚 Dependencies

- [`aes-gcm`](https://crates.io/crates/aes-gcm) – AES-GCM encryption  
- [`argon2`](https://crates.io/crates/argon2) – Key derivation from password  
- [`zxcvbn`](https://crates.io/crates/zxcvbn) – Password strength estimation  
- [`clap`](https://crates.io/crates/clap) – Command-line argument parsing  

---

## 🧑‍💻 Author

**Name**: Maciej Trznadel  
**Email**: maciej.trznadel24@gmail.com
**GitHub**: https://github.com/mtrznadel24