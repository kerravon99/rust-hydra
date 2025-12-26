use crate::cli::Args;

/// Parsed Hydra-style input
pub struct HydraArgs {
    pub protocol: String,
    pub target: String,
    pub users: String,
    pub passwords: String,
    pub threads: usize,
}

/// Detect and parse Hydra-style syntax
///
/// Examples:
///  - hydra -l admin -P pass.txt ssh://10.10.10.10
///  - hydra -L users.txt -P pass.txt ftp://10.10.10.10 -t 8
pub fn parse(argv: &[String]) -> Option<HydraArgs> {
    let mut user: Option<String> = None;
    let mut users_file: Option<String> = None;
    let mut pass_file: Option<String> = None;
    let mut threads: usize = 10;
    let mut target_url: Option<String> = None;

    let mut i = 1;
    while i < argv.len() {
        match argv[i].as_str() {
            "-l" => {
                user = Some(argv[i + 1].clone());
                i += 2;
            }
            "-L" => {
                users_file = Some(argv[i + 1].clone());
                i += 2;
            }
            "-P" => {
                pass_file = Some(argv[i + 1].clone());
                i += 2;
            }
            "-t" => {
                threads = argv[i + 1].parse().unwrap_or(10);
                i += 2;
            }
            arg if arg.contains("://") => {
                target_url = Some(arg.to_string());
                i += 1;
            }
            _ => i += 1,
        }
    }

    let target_url = target_url?;
    let pass_file = pass_file?;

    let (protocol, target) = split_url(&target_url)?;

    let users = if let Some(u) = user {
        let tmp = "/tmp/rust_hydra_single_user.txt";
        std::fs::write(tmp, format!("{}\n", u)).ok()?;
        tmp.to_string()
    } else {
        users_file?
    };

    Some(HydraArgs {
        protocol,
        target,
        users,
        passwords: pass_file,
        threads,
    })
}

fn split_url(url: &str) -> Option<(String, String)> {
    let parts: Vec<&str> = url.split("://").collect();
    if parts.len() != 2 {
        return None;
    }
    Some((parts[0].to_string(), parts[1].to_string()))
}
