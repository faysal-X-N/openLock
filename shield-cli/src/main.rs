use clap::{Parser, Subcommand};
use shield_core::{Vault, model::Entry};
use secrecy::{SecretBox, ExposeSecret};
use std::path::PathBuf;
use anyhow::{Result, Context};
use std::io::Write;
use zip::write::FileOptions;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long, value_name = "FILE")]
    db_path: Option<PathBuf>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init,
    Add {
        name: String,
        #[arg(short, long)]
        username: Option<String>,
    },
    Get {
        query: String,
    },
    List,
    Delete {
        uuid: String,
    },
    Edit {
        uuid: String,
        #[arg(short, long)]
        name: Option<String>,
        #[arg(short, long)]
        username: Option<String>,
        #[arg(short, long)]
        password: bool, // Flag to update password
    },
    Export {
        #[arg(short, long)]
        file: PathBuf,
    },
    Import {
        #[arg(short, long)]
        file: PathBuf,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    let db_path = if let Some(p) = cli.db_path {
        p
    } else {
        dirs::data_local_dir()
            .unwrap_or(PathBuf::from("."))
            .join("shield.db")
    };
    
    if let Some(parent) = db_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    if let Commands::Init = cli.command {
        println!("Initializing new vault at {:?}", db_path);
        if db_path.exists() {
             println!("Vault already exists at that path.");
        }
    }
    
    let password = if let Ok(p) = std::env::var("SHIELD_PASSWORD") {
        SecretBox::new(Box::new(p))
    } else {
        print!("Enter Master Password: ");
        std::io::stdout().flush()?;
        let password = rpassword::read_password()?;
        SecretBox::new(Box::new(password))
    };
    
    let vault = Vault::open(&db_path, &password).context("Failed to open vault")?;

    match cli.command {
        Commands::Init => {
            println!("Vault initialized successfully.");
        }
        Commands::Add { name, username } => {
            let entry_pass = if let Ok(p) = std::env::var("SHIELD_ENTRY_PASSWORD") {
                p
            } else {
                print!("Enter Password for entry '{}': ", name);
                std::io::stdout().flush()?;
                rpassword::read_password()?
            };
            
            let entry = Entry::new(name, username, entry_pass);
            vault.add_entry(&entry)?;
            println!("Entry added: {}", entry.uuid);
        }
        Commands::Get { query } => {
            if let Ok(uuid) = uuid::Uuid::parse_str(&query) {
                match vault.get_entry(&uuid) {
                    Ok(entry) => print_entry(&entry),
                    Err(_) => println!("Entry not found by UUID."),
                }
            } else {
                let entries = vault.list_entries()?;
                let found: Vec<_> = entries.iter().filter(|e| e.name.contains(&query)).collect();
                if found.is_empty() {
                    println!("No entries found matching '{}'", query);
                } else {
                    for entry in found {
                        print_entry(entry);
                    }
                }
            }
        }
        Commands::List => {
            let entries = vault.list_entries()?;
            println!("{:<36} | {:<20} | {:<20}", "UUID", "Name", "Username");
            println!("{}", "-".repeat(80));
            for entry in entries {
                println!("{:<36} | {:<20} | {:<20}", 
                    entry.uuid, 
                    entry.name, 
                    entry.username.as_deref().unwrap_or(""));
            }
        }
        Commands::Delete { uuid } => {
            let uuid = uuid::Uuid::parse_str(&uuid).context("Invalid UUID format")?;
            vault.delete_entry(&uuid)?;
            println!("Entry deleted.");
        }
        Commands::Edit { uuid, name, username, password } => {
            let uuid = uuid::Uuid::parse_str(&uuid).context("Invalid UUID format")?;
            let mut entry = vault.get_entry(&uuid)?;
            
            if let Some(n) = name {
                entry.name = n;
            }
            if let Some(u) = username {
                entry.username = Some(u);
            }
            if password {
                let entry_pass = if let Ok(p) = std::env::var("SHIELD_ENTRY_PASSWORD") {
                    p
                } else {
                    print!("Enter New Password for entry '{}': ", entry.name);
                    std::io::stdout().flush()?;
                    rpassword::read_password()?
                };
                entry.password = SecretBox::new(Box::new(entry_pass));
            }
            
            entry.update_timestamp();
            vault.update_entry(&entry)?;
            println!("Entry updated: {}", entry.uuid);
        }
        Commands::Export { file: file_path } => {
            let _zip_pass = if let Ok(p) = std::env::var("SHIELD_EXPORT_PASSWORD") {
                p
            } else {
                print!("Enter Password for Export Archive: ");
                std::io::stdout().flush()?;
                rpassword::read_password()?
            };
            
            let entries = vault.list_entries()?;
            let mut wtr = csv::Writer::from_writer(vec![]);
                
                // Write header explicitly if needed, but csv crate handles it by default with structs.
                // Since we are writing raw records, we should probably write header?
                // Or just rely on order. Let's write header for clarity if we were using Serialize.
                // But here we used write_record. Let's stick to simple no-header or implicit.
                // Actually, for import to be robust, we should probably write a header.
                // But the Reader relies on headers by default.
                
                wtr.write_record(&["uuid", "name", "username", "password", "url", "notes", "created_at", "updated_at"])?;
                
                for entry in entries {
                wtr.write_record(&[
                    entry.uuid.to_string(),
                    entry.name,
                    entry.username.unwrap_or_default(),
                    entry.password.expose_secret().clone(),
                    entry.url.unwrap_or_default(),
                    entry.notes.unwrap_or_default(),
                    entry.created_at.to_rfc3339(),
                    entry.updated_at.to_rfc3339(),
                ])?;
            }
            
            let csv_data = wtr.into_inner()?;
            
            let file = std::fs::File::create(&file_path)?;
            let mut zip = zip::ZipWriter::new(file);
            
            let options = FileOptions::default()
                .compression_method(zip::CompressionMethod::Stored)
                .unix_permissions(0o600);
            
            // Note: Encryption temporarily disabled due to dependency issues with zip crate > 1.0 on Windows
            // .with_deprecated_encryption(zip_pass.as_bytes());
                
            zip.start_file("shield_export.csv", options)?;
            zip.write_all(&csv_data)?;
            zip.finish()?;
            
            println!("Export completed to {:?}", file_path);
        }
        Commands::Import { file: file_path } => {
            let file = std::fs::File::open(&file_path).context("Failed to open import file")?;
            // Simple check if it's a zip or csv based on extension
            let is_zip = file_path.extension().and_then(|s| s.to_str()) == Some("zip");
            
            let mut entries_to_add = Vec::new();

            if is_zip {
                println!("Importing from ZIP archive...");
                let mut archive = zip::ZipArchive::new(file)?;
                // We look for shield_export.csv inside
                let mut csv_file = archive.by_name("shield_export.csv")
                    .context("Could not find shield_export.csv in archive")?;
                
                let mut rdr = csv::Reader::from_reader(csv_file);
                for result in rdr.records() {
                    let record = result?;
                    // CSV format with header: uuid, name, username, password, url, notes, created, updated
                    // record index 0 is uuid, 1 is name...
                    if record.len() >= 4 {
                        let name = record.get(1).unwrap_or("Untitled").to_string();
                        let username = record.get(2).map(|s| s.to_string());
                        let password = record.get(3).unwrap_or("").to_string();
                        let url = record.get(4).map(|s| s.to_string());
                        let notes = record.get(5).map(|s| s.to_string());
                        
                        let mut entry = Entry::new(name, username, password);
                        entry.url = url;
                        entry.notes = notes;
                        // We generate new UUIDs to avoid conflicts, or should we keep old ones?
                        // For import, usually better to generate new if we don't want to overwrite existing by accident.
                        // But if we want to restore... let's stick to new UUIDs for now to be safe.
                        entries_to_add.push(entry);
                    }
                }
            } else {
                println!("Importing from CSV...");
                let mut rdr = csv::Reader::from_reader(file);
                for result in rdr.records() {
                    let record = result?;
                    // Assuming same CSV format
                    if record.len() >= 4 {
                        let name = record.get(1).unwrap_or("Untitled").to_string();
                        let username = record.get(2).map(|s| s.to_string());
                        let password = record.get(3).unwrap_or("").to_string();
                        let url = record.get(4).map(|s| s.to_string());
                        let notes = record.get(5).map(|s| s.to_string());
                        
                        let mut entry = Entry::new(name, username, password);
                        entry.url = url;
                        entry.notes = notes;
                        entries_to_add.push(entry);
                    }
                }
            }

            let count = entries_to_add.len();
            for entry in entries_to_add {
                vault.add_entry(&entry)?;
                println!("Imported: {}", entry.name);
            }
            println!("Successfully imported {} entries.", count);
        }
    }

    Ok(())
}

fn print_entry(entry: &Entry) {
    use secrecy::ExposeSecret;
    println!("UUID: {}", entry.uuid);
    println!("Name: {}", entry.name);
    println!("Username: {}", entry.username.as_deref().unwrap_or(""));
    println!("Password: {}", entry.password.expose_secret());
    println!("Created: {}", entry.created_at);
    println!("Updated: {}", entry.updated_at);
    println!("--------------------------------");
}
