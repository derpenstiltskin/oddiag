pub mod util;
pub mod client;
pub mod ntfs;
pub mod scan;

use client::*;
use scan::*;

use anyhow::Result;
use clap::{arg, Command};

fn main() -> Result<()> {
    let matches = Command::new("oddiag")
        .version("0.1.0")
        .author("Dustin Riley <dustin@derpenstiltskin.com")
        .about("OneDrive utility written in Rust.")
        .arg(
            arg!(-a --account <USERNAME> "Scopes backup and report to specified user account")
            .required(false)
        )
        .arg(
            arg!(-b --backup <PATH> "Backup local saved OneDrive files (preserves folder structure)")
            .required(false)
        )
        .arg(
            arg!(-r --report <PATH> "Generate CSV report of local saved OneDrive files")
            .required(false)
        )
        .get_matches();
    
    let mut client = Client::new();
    let _ = if matches.is_present("account") {
        let username = matches.try_get_one::<String>("account")?.unwrap();
        client.scan(Some(username))
    } else {
        client.scan(None)
    };

    let client_version = client.get_version();
    let client_install_path = client.get_install_path();

    let client_business_accounts = client.get_business_accounts();

    println!("# ONEDRIVE APP ####################");
    println!("Version: {}", client_version);
    println!("Install Path: {}", client_install_path);

    if client_business_accounts.len() > 0 {
        for client_business_account in client_business_accounts {
            println!("# ACCOUNT #########################");
            println!("Username: {}", client_business_account.get_username());
            println!("Path: {}", client_business_account.get_path());
            println!("Tenant Id: {}", client_business_account.get_tenant_id());
            println!("Tenant Name: {}", client_business_account.get_tenant_name());
    
            if matches.is_present("backup") || matches.is_present("report") {
                let mut scan = Scan::new(client_business_account.get_path());
    
                scan.scan()?;
    
                println!("Total Local File Size: {} bytes", scan.get_size());
                println!("Total Local File Count: {}", scan.get_count());
                    
                if matches.is_present("backup") {
                    let backup_path = matches.try_get_one::<String>("backup")?.unwrap();
                    scan.backup(&backup_path)?;
                    println!("Backup created in: {}", &backup_path);
                }
        
                if matches.is_present("report") {
                    let report_path = format!("{}{}.csv", matches.try_get_one::<String>("report")?.unwrap(), client_business_account.get_username());
                    scan.report(&report_path)?;
                    println!("Report created: {}", &report_path);
                }
            }
        }
    } else {
        println!("# ACCOUNT #########################");
        println!("No accounts found.");
    }

    println!("###################################");

    Ok(())
}