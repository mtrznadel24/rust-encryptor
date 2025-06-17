use clap::{ArgGroup, Args, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    /// Wybierz jedną z dostępnych komend: encrypt, decrypt lub generate-key
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands{
    /// Szyfruje podany plik
    Encrypt {
        #[command(flatten)]
        args: EncryptorArgs,
    },
    /// Odszyfrowuje podany plik
    Decrypt {
        #[command(flatten)]
        args: EncryptorArgs,
    },
    /// Generuje losowy klucz i zapisuje go do pliku
    GenerateKey {
        /// Ścieżka do pliku, w którym zostanie zapisany wygenerowany klucz
        #[arg(short, long)]
        output: PathBuf,
    }
}

#[derive(Args)]
#[command(group(
    ArgGroup::new("auth")
    .required(true)
    .args(["password", "key"])
))]
pub struct EncryptorArgs{
    /// Ścieżka do pliku do zaszyfrowania lub odszyfrowania
    #[arg(short, long)]
    pub file:PathBuf,
    /// Hasło do wygenerowania klucza szyfrowania
    #[arg(short, long)]
    pub password: Option<String>,
    /// Ścieżka do pliku z kluczem szyfrowania
    #[arg(short, long)]
    pub key: Option<PathBuf>,
    /// Usuwa oryginalny plik po zakończeniu operacji
    #[arg(short, long="remove-original")]
    pub remove_original: bool
}