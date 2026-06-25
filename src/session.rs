use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

const EXPIRY: u64 = 86400;

fn path() -> PathBuf {
    let home =
        std::env::var("HOME")
            .unwrap();
    
    PathBuf::from(home)
        .join(".cache")
        .join("aurguard_session")
}

pub fn create_session(package: &str) {
    let file =
        path();
    
    if let Some(parent) =
        file.parent()
    {
        fs::create_dir_all(parent)
            .unwrap();
    }

    let data =
        format!(
            "{}\nCURRENT:{}\nLAST:{}\nAVAILABLE:{}\n",
            now(),
            package,
            package,
            package
        );
    
    fs::write(
        file,
        data
    )
    .unwrap();
}

pub fn replace_available(
    packages: &Vec<String>
) {
    cleanup();
    let file =
        path();

    let mut data =
        read()
            .unwrap_or_default();
    
    data = data
        .lines()
        .filter(|line| {
            !line.starts_with("AVAILABLE:")
        })
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join("\n");

    for package in packages {
        data.push_str(
            &format!(
                "\nAVAILABLE:{}",
                package
            )
        );
    }
    
    fs::write(
        file,
        data
    )
    .unwrap();
}

pub fn can_inspect(
    package: &str
) -> bool {
    cleanup();
    let data =
        read()
            .unwrap_or_default();
    
    data.lines()
        .any(|line| {
            line ==
            format!(
                "AVAILABLE:{}",
                package
            )
        })
}

pub fn set_current(
    package: &str
) {
    let file =
        path();

    let mut data =
        read()
            .unwrap()
            .lines()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
    
    for line in data.iter_mut() {
        if line.starts_with("CURRENT:") {
            *line =
                format!(
                    "CURRENT:{}",
                    package
                );
        }
    }
    fs::write(
        file,
        data.join("\n")
    )
    .unwrap();
}

pub fn set_last_with_dependencies(
    package: &str
) {
    let file =
        path();

    let mut data =
        read()
            .unwrap()
            .lines()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
    
    for line in data.iter_mut() {
        if line.starts_with("LAST:") {
            *line =
                format!(
                    "LAST:{}",
                    package
                );
        }
    }
    fs::write(
        file,
        data.join("\n")
    )
    .unwrap();
}

fn read() -> Option<String> {
    fs::read_to_string(path())
        .ok()
}

fn cleanup() {
    let file =
        path();

    if !file.exists() {
        return;
    }

    let data =
        read()
            .unwrap();

    let created: u64 =
        data.lines()
            .next()
            .unwrap()
            .parse()
            .unwrap();

    if now() - created > EXPIRY {
        fs::remove_file(file)
            .unwrap();
    }
}

fn now() -> u64 {
    SystemTime::now()
        .duration_since(
            UNIX_EPOCH
        )
        .unwrap()
        .as_secs()
}
