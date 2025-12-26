use crate::{
    protocol::Protocol,
    resume::ResumeState,
    stream::{stream_credentials, Credential},
};
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use std::sync::{
    atomic::{AtomicBool, AtomicUsize, Ordering},
    Arc,
};
use tokio::sync::{mpsc, Semaphore};

pub async fn run(
    proto: Arc<dyn Protocol>,
    args: crate::cli::Args,
    users: Vec<String>,
) {
    let resume = if args.resume {
        ResumeState::load()
    } else {
        None
    };

    let (tx, mut rx) = mpsc::channel::<Credential>(args.threads * 2);
    let semaphore = Arc::new(Semaphore::new(args.threads));
    let found = Arc::new(AtomicBool::new(false));
    let progress = Arc::new(AtomicUsize::new(
        resume.as_ref().map(|r| r.global_index).unwrap_or(0),
    ));

    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::with_template(
            "[{elapsed_precise}] {spinner} {pos} attempts | ETA {eta_precise}",
        )
        .unwrap(),
    );

    // PRODUCER
    {
        let tx = tx.clone();
        let password_file = args.passwords.clone();
        tokio::spawn(async move {
            for cred in stream_credentials(users, &password_file, resume) {
                if tx.send(cred).await.is_err() {
                    break;
                }
            }
        });
    }

    // CTRL-C SAVE
    {
        let progress = progress.clone();
        tokio::spawn(async move {
            tokio::signal::ctrl_c().await.unwrap();
            ResumeState {
                user_index: 0,
                password_line: 0,
                global_index: progress.load(Ordering::Relaxed),
            }
            .save();
            println!("\n{}", "[*] Progress saved".yellow());
            std::process::exit(0);
        });
    }

    while let Some(cred) = rx.recv().await {
        if args.stop_on_success && found.load(Ordering::Relaxed) {
            break;
        }

        let permit = semaphore.clone().acquire_owned().await.unwrap();
        let proto = proto.clone();
        let target = args.target.clone();
        let pb = pb.clone();
        let progress = progress.clone();
        let found = found.clone();

        tokio::spawn(async move {
            let _permit = permit;

            match proto
                .authenticate(&target, &cred.user, &cred.password)
                .await
            {
                Ok(true) => {
                    println!(
                        "{}",
                        format!("[+] {}:{}", cred.user, cred.password).green()
                    );
                    found.store(true, Ordering::Relaxed);
                }
                _ => {}
            }

            progress.fetch_add(1, Ordering::Relaxed);
            pb.inc(1);
        });
    }

    pb.finish_with_message("done");
}
