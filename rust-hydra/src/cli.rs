use clap::Parser;

#[derive(Parser, Clone)]
pub struct Args {
    pub protocol: String,
    pub target: String,

    #[arg(short = 'L')]
    pub users: String,

    #[arg(short = 'P')]
    pub passwords: String,

    #[arg(short = 't', default_value = "2")]
    pub threads: usize,

    #[arg(long)]
    pub resume: bool,

    #[arg(long)]
    pub stop_on_success: bool,

    #[arg(long)]
    pub domain: Option<String>,

    #[arg(long)]
    pub ssl: bool,

    #[arg(long)]
    pub user_field: Option<String>,

    #[arg(long)]
    pub pass_field: Option<String>,

    #[arg(long)]
    pub fail: Option<String>,
}
