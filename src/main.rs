mod parser;
mod mask;
mod diff;
mod validator;
mod crypto;
mod export;

use std::collections::BTreeMap;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process;
use clap::{Parser, Subcommand};
use colored::*;

#[derive(Parser, Debug)]
#[command(
    name = "rustenv",
    version,
    about = "A fast, secure environment variable and secret management tool — written in Rust"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Show {
        #[arg(default_value = ".env", help = "Path to the .env file")]
        file: String,

        #[arg(long, help = "Show sensitive values without masking")]
        unmask: bool,
    },
    Diff {
        #[arg(help = "Path to the first environment file")]
        file1: String,

        #[arg(help = "Path to the second environment file")]
        file2: String,

        #[arg(long, help = "Show sensitive values without masking in diff")]
        unmask: bool,
    },
    Validate {
        #[arg(default_value = ".env", help = "Path to the .env file")]
        file: String,
    },
    Export {
        #[arg(default_value = ".env", help = "Path to the .env file")]
        file: String,

        #[arg(long, default_value = "default", help = "Export format: default, shell, docker, json")]
        format: String,

        #[arg(long, help = "Show sensitive values without masking in export")]
        unmask: bool,
    },
    Encrypt {
        #[arg(help = "Path to the .env file to encrypt")]
        file: String,

        #[arg(short, long, help = "Output path for the encrypted file (defaults to <file>.enc)")]
        output: Option<String>,

        #[arg(short, long, help = "Password for encryption. If not provided, you will be prompted.")]
        password: Option<String>,
    },
    Decrypt {
        #[arg(help = "Path to the encrypted file to decrypt")]
        file: String,

        #[arg(short, long, help = "Output path for the decrypted file (defaults to .env)")]
        output: Option<String>,

        #[arg(short, long, help = "Password for decryption. If not provided, you will be prompted.")]
        password: Option<String>,
    },
    Merge {
        #[arg(help = "Path to the base environment file")]
        file1: String,

        #[arg(help = "Path to the environment file with override values")]
        file2: String,

        #[arg(short, long, help = "Output file path. If not provided, output will be printed to stdout.")]
        output: Option<String>,
    },
    Init {
        #[arg(long, default_value = ".env.template", help = "Template file path")]
        from: String,

        #[arg(short, long, default_value = ".env", help = "Output file path")]
        output: String,

        #[arg(long, help = "Prompt interactively for values in the template")]
        interactive: bool,
    },
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Show { file, unmask } => {
            let path = Path::new(&file);
            let vars = match parser::parse_env_file(path) {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("{}: {:?}", "Error".red().bold(), e);
                    process::exit(1);
                }
            };
            export::export_variables(&vars, "default", !unmask);
        }
        Commands::Diff { file1, file2, unmask } => {
            let p1 = Path::new(&file1);
            let p2 = Path::new(&file2);
            let vars1 = match parser::parse_env_file(p1) {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("{}: {:?}", "Error reading first file".red().bold(), e);
                    process::exit(1);
                }
            };
            let vars2 = match parser::parse_env_file(p2) {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("{}: {:?}", "Error reading second file".red().bold(), e);
                    process::exit(1);
                }
            };
            diff::print_diff(&vars1, &vars2, !unmask);
        }
        Commands::Validate { file } => {
            let path = Path::new(&file);
            let vars = match parser::parse_env_file(path) {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("{}: {:?}", "Error".red().bold(), e);
                    process::exit(1);
                }
            };
            let results = validator::validate_variables(&vars);
            if !validator::print_validation(&results) {
                process::exit(1);
            }
        }
        Commands::Export { file, format, unmask } => {
            let path = Path::new(&file);
            let vars = match parser::parse_env_file(path) {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("{}: {:?}", "Error".red().bold(), e);
                    process::exit(1);
                }
            };
            export::export_variables(&vars, &format, !unmask);
        }
        Commands::Encrypt { file, output, password } => {
            let path = Path::new(&file);
            let out_path_str = output.unwrap_or_else(|| format!("{}.enc", file));
            let out_path = Path::new(&out_path_str);

            let pass = password.unwrap_or_else(|| {
                print!("Enter encryption password: ");
                io::stdout().flush().unwrap();
                let mut p = String::new();
                io::stdin().read_line(&mut p).unwrap();
                p.trim().to_string()
            });

            if pass.is_empty() {
                eprintln!("{}", "Error: Password cannot be empty".red().bold());
                process::exit(1);
            }

            let data = match fs::read(path) {
                Ok(d) => d,
                Err(e) => {
                    eprintln!("{}: {:?}", "Error reading input file".red().bold(), e);
                    process::exit(1);
                }
            };

            match crypto::encrypt_data(&data, &pass) {
                Ok(enc) => {
                    if let Err(e) = fs::write(out_path, enc) {
                        eprintln!("{}: {:?}", "Error writing encrypted file".red().bold(), e);
                        process::exit(1);
                    }
                    println!("Successfully encrypted {:?} to {:?}", path, out_path);
                }
                Err(e) => {
                    eprintln!("{}: {:?}", "Encryption failed".red().bold(), e);
                    process::exit(1);
                }
            }
        }
        Commands::Decrypt { file, output, password } => {
            let path = Path::new(&file);
            let out_path_str = output.unwrap_or_else(|| ".env".to_string());
            let out_path = Path::new(&out_path_str);

            let pass = password.unwrap_or_else(|| {
                print!("Enter decryption password: ");
                io::stdout().flush().unwrap();
                let mut p = String::new();
                io::stdin().read_line(&mut p).unwrap();
                p.trim().to_string()
            });

            if pass.is_empty() {
                eprintln!("{}", "Error: Password cannot be empty".red().bold());
                process::exit(1);
            }

            let data = match fs::read(path) {
                Ok(d) => d,
                Err(e) => {
                    eprintln!("{}: {:?}", "Error reading encrypted file".red().bold(), e);
                    process::exit(1);
                }
            };

            match crypto::decrypt_data(&data, &pass) {
                Ok(dec) => {
                    if let Err(e) = fs::write(out_path, dec) {
                        eprintln!("{}: {:?}", "Error writing decrypted file".red().bold(), e);
                        process::exit(1);
                    }
                    println!("Successfully decrypted {:?} to {:?}", path, out_path);
                }
                Err(e) => {
                    eprintln!("{}: {:?}", "Decryption failed".red().bold(), e);
                    process::exit(1);
                }
            }
        }
        Commands::Merge { file1, file2, output } => {
            let p1 = Path::new(&file1);
            let p2 = Path::new(&file2);

            let mut vars1 = match parser::parse_env_file(p1) {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("{}: {:?}", "Error reading base file".red().bold(), e);
                    process::exit(1);
                }
            };

            let vars2 = match parser::parse_env_file(p2) {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("{}: {:?}", "Error reading override file".red().bold(), e);
                    process::exit(1);
                }
            };

            for (k, v) in vars2 {
                vars1.insert(k, v);
            }

            if let Some(out_file) = output {
                let out_path = Path::new(&out_file);
                if let Err(e) = parser::write_env_file(out_path, &vars1) {
                    eprintln!("{}: {:?}", "Error writing merged file".red().bold(), e);
                    process::exit(1);
                }
                println!("Successfully merged files into {:?}", out_path);
            } else {
                for (k, v) in &vars1 {
                    println!("{}={}", k, v);
                }
            }
        }
        Commands::Init { from, output, interactive } => {
            let template_path = Path::new(&from);
            let out_path = Path::new(&output);

            if !template_path.exists() {
                eprintln!("{}: Template file {:?} does not exist", "Error".red().bold(), template_path);
                process::exit(1);
            }

            let template_vars = match parser::parse_env_file(template_path) {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("{}: {:?}", "Error reading template file".red().bold(), e);
                    process::exit(1);
                }
            };

            let mut out_vars = BTreeMap::new();

            if interactive {
                println!("{}", "Interactive configuration initialization:".green().bold());
                for (k, v) in template_vars {
                    print!("{} [{}]? ", k.bold(), v.dimmed());
                    io::stdout().flush().unwrap();
                    let mut input = String::new();
                    io::stdin().read_line(&mut input).unwrap();
                    let input_trimmed = input.trim();
                    if input_trimmed.is_empty() {
                        out_vars.insert(k, v);
                    } else {
                        out_vars.insert(k, input_trimmed.to_string());
                    }
                }
            } else {
                out_vars = template_vars;
            }

            if let Err(e) = parser::write_env_file(out_path, &out_vars) {
                eprintln!("{}: {:?}", "Error writing initialized file".red().bold(), e);
                process::exit(1);
            }

            println!("Successfully initialized environment file at {:?}", out_path);
        }
    }
}
