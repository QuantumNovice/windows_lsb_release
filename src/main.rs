use std::env;
use winreg::enums::*;
use winreg::RegKey;

struct WindowsInfo {
    distributor_id: String,
    description: String,
    release: String,
    codename: String,
}

fn main() {
    let windows_info = get_windows_info().unwrap_or_else(|e| {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    });

    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        match args[1].as_str() {
            "-a" | "--all" => print_all(&windows_info),
            "-s" | "--short" => print_short(&windows_info),
            "-d" | "--description" => println!("{}", windows_info.description),
            "-r" | "--release" => println!("{}", windows_info.release),
            "-c" | "--codename" => println!("{}", windows_info.codename),
            "-i" | "--distributor-id" => println!("{}", windows_info.distributor_id),
            _ => {
                eprintln!("Unknown option: {}", args[1]);
                print_usage();
            }
        }
    } else {
        print_all(&windows_info);
    }
}

fn get_windows_info() -> Result<WindowsInfo, Box<dyn std::error::Error>> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let cur_ver = hklm.open_subkey("SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion")?;

    let product_name: String = cur_ver.get_value("ProductName")?;
    let display_version: String = cur_ver.get_value("DisplayVersion")?;
    let current_build: String = cur_ver.get_value("CurrentBuild")?;
    let release_id: String = cur_ver.get_value("ReleaseId")?;

    Ok(WindowsInfo {
        distributor_id: "Microsoft".to_string(),
        description: product_name,
        release: format!("{}.{}", display_version, current_build),
        codename: release_id,
    })
}

fn print_all(info: &WindowsInfo) {
    println!("Distributor ID:\t{}", info.distributor_id);
    println!("Description:\t{}", info.description);
    println!("Release:\t{}", info.release);
    println!("Codename:\t{}", info.codename);
}

fn print_short(info: &WindowsInfo) {
    println!("d\t{}", info.distributor_id);
    println!("p\t{}", info.description);
    println!("r\t{}", info.release);
    println!("c\t{}", info.codename);
}

fn print_usage() {
    println!("Usage: windows_release [options]");
    println!("Options:");
    println!("  -a, --all\t\tshow all information");
    println!("  -s, --short\t\tshow information in short format");
    println!("  -d, --description\tshow description only");
    println!("  -r, --release\t\tshow release only");
    println!("  -c, --codename\t\tshow codename only");
    println!("  -i, --distributor-id\tshow distributor ID only");
}
