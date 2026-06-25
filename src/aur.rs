use std::process::Command;
use std::io::{self, Write};
use crate::session;

const BLUE: &str = "\x1b[1;34m";
const RED: &str = "\x1b[1;31m";
const RESET: &str = "\x1b[0m";

pub fn scan_package(package: &str) {
    println!(
        "Scanning package '{}'...\n",
        package
    );
    if let Some(origin) = package_origin(package) {
        println!(
            "Origin: {}\n",
            origin
        );
    }
    let pkg =
        fetch_pkgbuild(package);

    if pkg.is_empty() {
        println!("Package not found.");
        return;
    }
    let deps =
        show_dependencies(&pkg);

    if deps.is_empty() {
        println!("No dependencies.");
        return;
    }
    println!("Dependencies:");
    print_dependencies(&deps);
    session::replace_available(&deps);
    print!(
        "\nScan dependencies? [y/N] "
    );
    io::stdout()
        .flush()
        .unwrap();

    let mut input =
        String::new();

    io::stdin()
        .read_line(&mut input)
        .unwrap();

    if input.trim()
        .eq_ignore_ascii_case("y")
    {
        println!(
            "\nChecking dependencies...\n"
        );
        for dep in deps {
            check_type(&dep);
        }
    }
}

pub fn inspect_package(package: &str) {
    println!(
        "Inspecting '{}'...\n",
        package
    );
    if let Some(origin) = package_origin(package) {
        println!(
            "Origin: {}\n",
            origin
        );
    }
    let pkg =
        fetch_pkgbuild(package);

    security_checks(&pkg);

    let deps =
        show_dependencies(&pkg);

    if deps.is_empty() {
        println!("\nNo dependencies.");
        return;
    }
    println!("\nDependencies:");
    print_dependencies(&deps);
    session::replace_available(&deps);
    session::set_current(package);
    session::set_last_with_dependencies(package);
}

fn package_origin(
    package: &str
) -> Option<String> {
    if arch_package_exists(package) {
        return Some(
            format!(
                "https://archlinux.org/packages/search?q={}",
                package
            )
        );
    }

    if is_aur_package(package) {
        return Some(
            format!(
                "https://aur.archlinux.org/packages/{}",
                package
            )
        );
    }
    None
}

fn arch_package_exists(
    package: &str
) -> bool {
    let url =
        format!(
            "https://archlinux.org/packages/search?q={}",
            package
        );

    match Command::new("curl")
        .args([
            "-Ls",
            &url
        ])
        .output()
    {
        Ok(out) => {
            let page =
                String::from_utf8_lossy(
                    &out.stdout
                );
            
            !page.contains("No results found")
            &&
            page.contains("package")
        }
        Err(_) =>
            false
    }
}

fn is_official_package(
    package: &str
) -> bool {
    match Command::new("pacman")
        .args(["-Si", package])
        .output()
    {
        Ok(out) =>
            out.status.success(),
        Err(_) =>
            false
    }
}

fn is_aur_package(
    package: &str
) -> bool {
    let pkg =
        fetch_aur(package);
    
    pkg.contains("pkgname=")
    && pkg.contains("pkgver=")
    && pkg.contains("pkgrel=")
}

fn check_type(
    package: &str
) {
    print!(
        "{}... ",
        package
    );
    if is_official_package(package) {
        println!(
            "{}Official Arch package{}",
            BLUE,
            RESET
        );
    }
    else if is_aur_package(package) {
        println!(
            "AUR package"
        );
    }
    else {
        println!(
            "{}Unknown{}",
            RED,
            RESET
        );
    }
}

fn print_dependencies(
    deps: &Vec<String>
) {
    for dep in deps {
        if is_official_package(dep) {
            println!(
                "{}- {}{}",
                BLUE,
                dep,
                RESET
            );
        }
        else if is_aur_package(dep) {
            println!(
                "- {}",
                dep
            );
        }
        else {
            println!(
                "{}- {}{}",
                RED,
                dep,
                RESET
            );
        }
    }
}

fn security_checks(
    text: &str
) {
    println!("Checks:");
    println!(
        "- Checksums: {}",
        yes_no(
            text.contains("sha256sums")
            || text.contains("b2sums")
            || text.contains("md5sums")
        )
    );
    println!(
        "- Downloads files: {}",
        yes_no(
            text.contains("curl")
            || text.contains("wget")
            || text.contains("fetch")
        )
    );
    println!(
        "- Executes remote scripts: {}",
        yes_no(
            text.contains("| bash")
            || text.contains("| sh")
            || (
                text.contains("curl")
                &&
                text.contains("bash")
            )
        )
    );
    println!(
        "- Modifies services: {}",
        yes_no(
            text.contains("systemctl")
            || text.contains("service ")
        )
    );
    println!(
        "- Modifies system files: {}",
        yes_no(
            text.contains("/etc/")
            || text.contains("/usr/")
            || text.contains("/bin/")
        )
    );
    println!(
        "- Uses dangerous commands: {}",
        yes_no(
            text.contains("rm -rf")
            || text.contains("chmod 777")
            || text.contains("chown")
        )
    );
}

fn yes_no(
    value: bool
) -> &'static str {
    if value {
        "y"
    } else {
        "n"
    }
}

fn show_dependencies(
    text: &str
) -> Vec<String> {
    let mut deps =
        Vec::new();
    
    let mut inside =
        false;
    
    for line in text.lines() {
        let line =
            line.trim();
        
        if line.starts_with("Depends On") {
            let list =
                line
                .split(':')
                .nth(1)
                .unwrap_or("")
                .split_whitespace();

            for dep in list {
                add_dep(&mut deps, dep);
            }
        }
        if line.starts_with("depends=(") {
            inside = true;
        }
        if inside {
            let clean =
                line
                .replace("depends=(", "")
                .replace(")", "")
                .replace("\"", "")
                .replace("'", "");

            for dep in clean.split_whitespace() {
                add_dep(&mut deps, dep);
            }
        }
        if line.ends_with(")") {
            inside = false;
        }
    }
    deps
}
fn add_dep(
    deps: &mut Vec<String>,
    dep: &str
) {
    let dep =
        dep
        .split(|c| c == '>' || c == '<' || c == '=')
        .next()
        .unwrap()
        .to_string();

    if !dep.is_empty()
        && dep != "None"
        && !deps.contains(&dep)
    {
        deps.push(dep);
    }
}

fn fetch_pkgbuild(
    package: &str
) -> String {
    if let Ok(out) =
        Command::new("pacman")
            .args(["-Si", package])
            .output()
    {
        if out.status.success() {
            return String::from_utf8_lossy(
                &out.stdout
            )
            .to_string();
        }
    }
    fetch_aur(package)
}

fn fetch_aur(
    package: &str
) -> String {
    let url =
        format!(
        "https://aur.archlinux.org/cgit/aur.git/plain/PKGBUILD?h={}",
        package
    );
    match Command::new("curl")
        .args(["-s", &url])
        .output()
    {
        Ok(out) =>
            String::from_utf8_lossy(
                &out.stdout
            )
            .to_string(),

        Err(_) =>
            String::new()
    }
}
