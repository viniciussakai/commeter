use console::style;
use dialoguer::{theme::ColorfulTheme, Input, Select};
use std::os::unix::process::CommandExt;
use std::process::Command;

fn main() {
    println!("Commeter: a commit tool \n");

    let selection_keys = &[
        "feat", "fix", "docs", "style", "refactor", "perf", "test", "chore",
    ];

    let selections = &[
        "feat:      A new feature",
        "fix:       A bug fix",
        "docs:      Documentation only changes",
        "style:     Change code style that do not affect the meaning of the code",
        "refactor:  Acode change that neither fix a bug or adds a feature",
        "perf:      A code change that improves performance",
        "test:      Adding missing tests",
        "chore:     Changes to the build process or auxiliary tools",
    ];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select the type of change that you're committing: ")
        .default(0)
        .items(&selections[..])
        .report(false)
        .interact()
        .unwrap();

    println!(
        "{} {} {}",
        style("✔").green(),
        style("Select the type of change that you're committing:").bold(),
        style(selection_keys[selection]).green()
    );

    let scope: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt(
            "Scope. Could be anything specifying place of commit on empty: (db, users, poll):\n",
        )
        .allow_empty(true)
        .interact()
        .unwrap();

    let subject: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt(
            "Subject. Concise description of changes. Imperative, lowercase and a final dot:\n",
        )
        .interact()
        .unwrap();

    let body: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Body. Motivation for the change and contrast with previous behavior:\n")
        .allow_empty(true)
        .interact()
        .unwrap();

    let footer: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Footer. Information about Breaking Changes and reference issues: \n")
        .allow_empty(true)
        .interact()
        .unwrap();

    let mut commit = selection_keys[selection].to_string();

    if !scope.is_empty() {
        commit.push('(');
        commit.push_str(&scope);
        commit.push(')');
    }

    commit.push_str(": ");
    commit.push_str(&subject);
    commit.push_str("\n");

    if !body.is_empty() {
        commit.push_str("\n");
        commit.push_str(&body);
        commit.push_str("\n");
    }

    if !footer.is_empty() {
        commit.push_str("\n");
        commit.push_str(&footer);
        commit.push_str("\n");
    }

    println!("\n");

    let mut git = Command::new("git");
    git.args(["commit", "-m", commit.as_str()]);
    git.exec();
}
