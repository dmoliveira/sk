use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::io::{self, Read, Write};
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::process::{Command, Output, Stdio};

use clap::{ArgAction, CommandFactory, Parser, Subcommand};

const APP_NAME: &str = "sk";

#[derive(Parser, Debug)]
#[command(
    name = "sk",
    about = "Minimal macOS Keychain CLI for storing and retrieving secrets by key",
    disable_version_flag = true,
    arg_required_else_help = true
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    #[arg(short = 'V', long = "version", global = true, action = ArgAction::SetTrue)]
    version: bool,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Add {
        #[arg(short, long)]
        key: String,
        #[arg(short, long)]
        value: Option<String>,
        #[arg(long, action = ArgAction::SetTrue)]
        stdin: bool,
        #[arg(short, long)]
        user: Option<String>,
        #[arg(short, long, action = ArgAction::SetTrue)]
        force: bool,
    },
    Get {
        #[arg(short, long)]
        key: String,
        #[arg(short, long)]
        user: Option<String>,
    },
    List {
        #[arg(short, long)]
        user: Option<String>,
        #[arg(long, action = ArgAction::SetTrue)]
        show: bool,
        #[arg(long, action = ArgAction::SetTrue)]
        keys: bool,
    },
    Remove {
        #[arg(short, long)]
        key: String,
        #[arg(short, long)]
        user: Option<String>,
        #[arg(short, long, action = ArgAction::SetTrue)]
        yes: bool,
    },
    Install,
    Uninstall,
    Selfcheck,
    Version,
}

fn main() {
    if let Err(err) = run() {
        eprintln!("Error: {err}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    ensure_darwin()?;
    let cli = Cli::parse();
    if cli.version {
        print_version();
        return Ok(());
    }

    match cli.command {
        Some(Commands::Add {
            key,
            value,
            stdin,
            user,
            force,
        }) => cmd_add(&key, value, stdin, &default_user(user), force),
        Some(Commands::Get { key, user }) => cmd_get(&key, &default_user(user)),
        Some(Commands::List {
            user,
            show,
            keys: _,
        }) => {
            let keys_only = !show;
            cmd_list(&default_user(user), keys_only)
        }
        Some(Commands::Remove { key, user, yes }) => cmd_remove(&key, &default_user(user), yes),
        Some(Commands::Install) => cmd_install(),
        Some(Commands::Uninstall) => cmd_uninstall(),
        Some(Commands::Selfcheck) => cmd_selfcheck(),
        Some(Commands::Version) => {
            print_version();
            Ok(())
        }
        None => {
            Cli::command().print_help().map_err(|e| e.to_string())?;
            println!();
            Ok(())
        }
    }
}

fn ensure_darwin() -> Result<(), String> {
    if env::consts::OS != "macos" {
        return Err("sk is supported on macOS only".to_string());
    }
    Ok(())
}

fn default_user(user: Option<String>) -> String {
    user.or_else(|| env::var("SK_USER").ok())
        .or_else(|| env::var("USER").ok())
        .unwrap_or_else(|| "unknown".to_string())
}

fn service_prefix() -> String {
    env::var("SK_SERVICE_PREFIX").unwrap_or_else(|_| "sk:".to_string())
}

fn service_name(key: &str) -> String {
    format!("{}{}", service_prefix(), key)
}

fn cmd_add(
    key: &str,
    value: Option<String>,
    stdin: bool,
    user: &str,
    force: bool,
) -> Result<(), String> {
    validate_key(key)?;

    if value.is_some() && stdin {
        return Err("Use --value VALUE or --stdin, not both".to_string());
    }

    let mut resolved = value.unwrap_or_default();
    if stdin || resolved == "-" {
        resolved = read_secret_from_stdin()?;
    }

    if resolved.is_empty() {
        return Err("add requires --value VALUE or --stdin".to_string());
    }

    if key_exists(user, &service_name(key))? && !force {
        return Err(format!("Key exists: {key} (use --force to overwrite)"));
    }

    let service = service_name(key);
    let mut args = vec![
        "add-generic-password",
        "-a",
        user,
        "-s",
        &service,
        "-w",
        &resolved,
    ];
    if force {
        args.push("-U");
    }
    run_security(&args)?;

    println!("Stored key \"{key}\" for user \"{user}\"");
    Ok(())
}

fn cmd_get(key: &str, user: &str) -> Result<(), String> {
    validate_key(key)?;
    let output = run_security(&[
        "find-generic-password",
        "-a",
        user,
        "-s",
        &service_name(key),
        "-w",
    ])?;
    print_stdout(output);
    Ok(())
}

fn cmd_list(user: &str, keys_only: bool) -> Result<(), String> {
    let services = list_services()?;
    if services.is_empty() {
        println!("No sk entries found.");
        return Ok(());
    }

    let prefix = service_prefix();
    let mut printed = 0usize;
    for service in services {
        let Some(key) = service.strip_prefix(&prefix) else {
            continue;
        };

        if keys_only {
            println!("{key}");
            printed += 1;
            continue;
        }

        let maybe_value =
            run_security(&["find-generic-password", "-a", user, "-s", &service, "-w"]);
        if let Ok(output) = maybe_value {
            let value = String::from_utf8_lossy(&output.stdout).trim().to_string();
            println!("{key}\t{}", mask_value(&value));
            printed += 1;
        }
    }

    if printed == 0 {
        println!("No sk entries found for user \"{user}\".");
    }
    Ok(())
}

fn cmd_remove(key: &str, user: &str, yes: bool) -> Result<(), String> {
    validate_key(key)?;
    if !yes {
        print!("Remove key '{key}' for user '{user}'? [y/N] ");
        io::stdout().flush().map_err(|e| e.to_string())?;
        let mut answer = String::new();
        io::stdin()
            .read_line(&mut answer)
            .map_err(|e| e.to_string())?;
        if !matches!(answer.trim(), "y" | "Y") {
            return Err("Aborted.".to_string());
        }
    }

    run_security(&[
        "delete-generic-password",
        "-a",
        user,
        "-s",
        &service_name(key),
    ])?;
    println!("Removed key \"{key}\" for user \"{user}\"");
    Ok(())
}

fn cmd_install() -> Result<(), String> {
    let exe = env::current_exe().map_err(|e| e.to_string())?;
    let install_dir = env::var("SK_INSTALL_DIR").unwrap_or_else(|_| {
        format!(
            "{}/.local/bin",
            env::var("HOME").unwrap_or_else(|_| ".".to_string())
        )
    });
    let target = PathBuf::from(&install_dir).join(APP_NAME);

    if exe == target {
        println!("{APP_NAME} is already installed at {}", target.display());
        return Ok(());
    }

    fs::create_dir_all(&install_dir).map_err(|e| e.to_string())?;
    fs::copy(&exe, &target).map_err(|e| e.to_string())?;

    let mut perms = fs::metadata(&target)
        .map_err(|e| e.to_string())?
        .permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&target, perms).map_err(|e| e.to_string())?;

    println!("Installed {APP_NAME} to {}", target.display());
    if !env::var("PATH")
        .unwrap_or_default()
        .split(':')
        .any(|entry| entry == install_dir)
    {
        println!("Add {install_dir} to your PATH: export PATH=\"{install_dir}:$PATH\"");
    }
    Ok(())
}

fn cmd_uninstall() -> Result<(), String> {
    let install_dir = env::var("SK_INSTALL_DIR").unwrap_or_else(|_| {
        format!(
            "{}/.local/bin",
            env::var("HOME").unwrap_or_else(|_| ".".to_string())
        )
    });
    let target = PathBuf::from(&install_dir).join(APP_NAME);
    if target.exists() {
        fs::remove_file(&target).map_err(|e| e.to_string())?;
        println!("Removed {APP_NAME} from {}", target.display());
    } else {
        println!("No install found at {}", target.display());
    }
    Ok(())
}

fn cmd_selfcheck() -> Result<(), String> {
    let user = default_user(None);
    let key = format!("sk-selfcheck-{}", std::process::id());
    let service = service_name(&key);
    let probe = "ok";

    if key_exists(&user, &service)? {
        let _ = run_security(&["delete-generic-password", "-a", &user, "-s", &service]);
    }

    run_security(&[
        "add-generic-password",
        "-a",
        &user,
        "-s",
        &service,
        "-w",
        probe,
    ])?;

    let output = run_security(&["find-generic-password", "-a", &user, "-s", &service, "-w"])?;

    let found = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if found != probe {
        return Err("Unexpected value from Keychain".to_string());
    }

    run_security(&["delete-generic-password", "-a", &user, "-s", &service])?;
    println!("Keychain access OK for user \"{user}\"");
    Ok(())
}

fn print_version() {
    println!("{APP_NAME} {}", env!("CARGO_PKG_VERSION"));
}

fn validate_key(key: &str) -> Result<(), String> {
    if key.is_empty() {
        return Err("Key cannot be empty".to_string());
    }
    if key.chars().any(char::is_whitespace) {
        return Err("Key cannot contain whitespace".to_string());
    }
    Ok(())
}

fn read_secret_from_stdin() -> Result<String, String> {
    let mut buf = String::new();
    io::stdin()
        .read_to_string(&mut buf)
        .map_err(|e| e.to_string())?;
    let value = buf.trim_end_matches(['\n', '\r']).to_string();
    if value.is_empty() {
        return Err("No value provided on stdin".to_string());
    }
    Ok(value)
}

fn mask_value(value: &str) -> String {
    if value.is_empty() {
        return "****".to_string();
    }
    if value.chars().count() <= 4 {
        return format!("{value}****");
    }
    let prefix: String = value.chars().take(4).collect();
    format!("{prefix}****")
}

fn list_services() -> Result<Vec<String>, String> {
    let output = run_security(&["dump-keychain"])?;
    let raw = String::from_utf8_lossy(&output.stdout);
    let prefix = service_prefix();
    let mut set = BTreeSet::new();

    for line in raw.lines() {
        if !line.contains("\"svce\"<blob>=\"") {
            continue;
        }
        if let Some(start) = line.find("\"svce\"<blob>=\"") {
            let head = &line[(start + 14)..];
            if let Some(end) = head.find('\"') {
                let service = head[..end].to_string();
                if service.starts_with(&prefix) {
                    set.insert(service);
                }
            }
        }
    }

    Ok(set.into_iter().collect())
}

fn key_exists(user: &str, service: &str) -> Result<bool, String> {
    let status = Command::new("security")
        .arg("find-generic-password")
        .arg("-a")
        .arg(user)
        .arg("-s")
        .arg(service)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map_err(|e| e.to_string())?;
    Ok(status.success())
}

fn run_security(args: &[&str]) -> Result<Output, String> {
    let output = Command::new("security")
        .args(args)
        .output()
        .map_err(|e| format!("Missing dependency: security ({e})"))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        if stderr.is_empty() {
            return Err("security command failed".to_string());
        }
        return Err(stderr);
    }
    Ok(output)
}

fn print_stdout(output: Output) {
    print!("{}", String::from_utf8_lossy(&output.stdout));
}

#[cfg(test)]
mod tests {
    use super::{mask_value, validate_key};

    #[test]
    fn validates_key_rules() {
        assert!(validate_key("OPENAI_API_KEY").is_ok());
        assert!(validate_key("").is_err());
        assert!(validate_key("bad key").is_err());
    }

    #[test]
    fn masks_values_consistently() {
        assert_eq!(mask_value(""), "****");
        assert_eq!(mask_value("abcd"), "abcd****");
        assert_eq!(mask_value("abcdefgh"), "abcd****");
    }
}
