use console::{style, Term};
use notify_rust::{Notification as SystemNotification, Timeout};
use crate::repository::GithubRepository;
use std::error::Error;

pub struct TerminalDisplay {
    term: Term,
}

impl TerminalDisplay {
    pub fn new() -> Self {
        Self {
            term: Term::stdout(),
        }
    }

    pub fn config_loaded(&self) {
        println!(
            "{} Configuration loaded successfully",
            style("[✓]").green().bold()
        );
    }

    pub fn empty_config(&self) {
        println!("{} Notification config is empty!", style("[!]").yellow().bold());
    }

    pub fn inspect(&self, repo: &GithubRepository) {
        let header = format!(
            "Inspecting repository {} ...",
            style(repo.uri()).white().underlined()
        );
        println!("{} {}", style("[*]").blue().bold(), header);
    }

    pub fn repository_error(&self, error: &str) {
        eprintln!("{} {}", style("✗").red().bold(), error);
    }

    pub fn repository_added(&self, repo: &GithubRepository) -> Result<(), Box<dyn Error>> {
        self.clear_lines(1)?;
        println!(
            "{} Repository {} has been added to the database",
            style("[*]").blue().bold(),
            style(repo.uri()).white().underlined()
        );
        Ok(())
    }

    pub fn downloading_commits(&self) {
        println!(
            "{:>4}{} Downloading new commits ...",
            "",
            style("[*]").blue().bold()
        );
    }

    pub fn commit_info(
        &self,
        commit_url: &str,
        commit_sha: &str,
        filename: &str,
        additions: u64,
        deletions: u64,
    ) {
        // make the commit's hash a clickable link to the official github page
        let commit_to_link = format!(
            "\x1B]8;;{}\x07{}\x1B]8;;\x07",
            commit_url,
            commit_sha
        );
        
        println!(
            "{:>4}{} Commit SHA: {}",
            "",
            style("[!]").yellow().bold(),
            style(commit_to_link).blue().underlined(),
        );
        println!(
            "{:>7} File: {file}, Additions: {additions}, Deletions: {deletions}",
            "",
            file = style(filename).white().bold(),
            additions = style(additions).green().underlined(),
            deletions = style(deletions).red().underlined(),
        );
    }

    pub fn commit_notification(
        &self,
        repo_uri: &str,
        commit_sha: &str,
        author: &str,
    ) -> Result<(), Box<dyn Error>> {
        #[cfg(all(unix))]
        static SOUND: &str = "message-new-instant";
        #[cfg(target_os = "windows")]
        static SOUND: &str = "Mail";

        let summary = format!("🔍 New matching commit in {0}", repo_uri);
        let body: String = format!("👤 {0}\n🔗 {1}", author, commit_sha);
        
        SystemNotification::new()
            .summary(&summary)
            .body(&body)
            .sound_name(SOUND)
            .timeout(Timeout::Never)
            .show()?;
            
        Ok(())
    }

    pub fn clear_lines(&self, count: usize) -> Result<(), Box<dyn Error>> {
        self.term.clear_last_lines(count)?;
        Ok(())
    }
}

impl Default for TerminalDisplay {
    fn default() -> Self {
        Self::new()
    }
}