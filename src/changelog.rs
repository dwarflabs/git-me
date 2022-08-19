//------------------------------------------------------------------------------
// from+git_me@luketitley.com
//------------------------------------------------------------------------------
const CHANGELOG: &'static str = "changelog";

use maplit::hashmap;
use std::collections::HashMap;

type Work = HashMap<std::string::String, std::vec::Vec<std::string::String>>;

#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[allow(non_snake_case)]
struct Changelog {
    pub Artists: Work,
    pub Technical: Work,
}

impl Changelog {
    pub fn new() -> Self {
        Self {
            Artists: hashmap! {
                "General".to_string() => vec!["".to_string()]
            },
            Technical: hashmap! {
                "General".to_string() => vec!["".to_string()]
            },
        }
    }
    pub fn empty() -> Self {
        Self {
            Artists: HashMap::new(),
            Technical: HashMap::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.Artists.is_empty() && self.Technical.is_empty()
    }

    pub fn contains_entries(&self) -> bool {
        for (_, r) in self.Artists.iter() {
            if contains_something(r) {
                return true;
            }
        }
        for (_, r) in self.Technical.iter() {
            if contains_something(r) {
                return true;
            }
        }

        false
    }
}

//------------------------------------------------------------------------------
fn contains_something(lines: &[String]) -> bool {
    for line in lines.iter() {
        for c in line.chars() {
            if !c.is_whitespace() {
                return true;
            }
        }
    }

    false
}

//------------------------------------------------------------------------------
pub fn resolve(
    name: &str,
    repo_path: Option<&std::path::Path>,
) -> std::path::PathBuf {
    let mut changelog_file = if let Some(repo_path) = repo_path {
        repo_path.join(&std::path::PathBuf::from(CHANGELOG))
    } else {
        std::path::PathBuf::from(CHANGELOG)
    };
    changelog_file.push(std::path::Path::new(name));
    changelog_file.set_extension("yml");
    changelog_file
}

//------------------------------------------------------------------------------
pub fn create_stub(
    name: &str,
    repo_path: Option<&std::path::Path>,
) -> std::path::PathBuf {
    // Build the changelog file path
    let changelog_file = resolve(name, repo_path);

    // Make sure the owning folder exists
    std::fs::create_dir_all(changelog_file.parent().unwrap())
        .expect("Unable to create changlog folder");

    // Write a stub changelog file to disk
    use std::io::Write as _;
    serde_yaml::to_writer(
        std::fs::File::create(&changelog_file)
            .expect("Unable to create changelog file"),
        &Changelog::new(),
    )
    .expect("Unable to write the changlog to disk");

    changelog_file
}

//------------------------------------------------------------------------------
pub fn create_with_msg(
    name: &str,
    repo_path: Option<&std::path::Path>,
    msg: &str,
) -> std::path::PathBuf {
    // Build the changelog file path
    let changelog_file = resolve(name, repo_path);

    // Make sure the owning folder exists
    std::fs::create_dir_all(changelog_file.parent().unwrap())
        .expect("Unable to create changlog folder");

    // Change the changelog file
    let mut changelog = Changelog::new();
    changelog.Artists = hashmap! {
        "General".to_string() => vec![msg.to_string()]
    };

    // Write a stub changelog file to disk
    use std::io::Write as _;
    serde_yaml::to_writer(
        std::fs::File::create(&changelog_file)
            .expect("Unable to create changelog file"),
        &changelog,
    )
    .expect("Unable to write the changlog to disk");

    changelog_file
}

//------------------------------------------------------------------------------
pub fn verify(name: &str, repo_path: Option<&std::path::Path>) -> bool {
    validate(&resolve(name, repo_path))
}

//------------------------------------------------------------------------------
pub fn read(path: &std::path::PathBuf) -> std::string::String {
    use std::io::Read as _;
    let mut changelog_file = std::fs::File::open(
        path.to_str().expect("changelog path is not unicode"),
    )
    .expect(&format!(
        "Unable to read change log file '{}'",
        path.to_str().unwrap()
    ));

    let mut contents = String::new();
    changelog_file
        .read_to_string(&mut contents)
        .expect("Unable to read changelog for validation");

    contents
}

//------------------------------------------------------------------------------
pub fn read_formatted(path: &std::path::PathBuf) -> std::string::String {
    let change_log: Changelog = serde_yaml::from_reader(
        std::fs::File::open(
            path.to_str().expect("changelog path is not unicode"),
        )
        .expect(&format!(
            "Unable to read change log file '{}'",
            path.to_str().unwrap()
        )),
    )
    .expect(&format!(
        "Unable to parse the change log from disk '{}'",
        path.to_str().unwrap()
    ));

    let mut result = std::string::String::new();

    // Artists
    result.push_str("## Artists\n");
    for (title, lines) in change_log.Artists.iter() {
        if !(lines.is_empty() || lines[0].is_empty()) {
            result.push_str("### ");
            result.push_str(&title);
            result.push_str("\n\n");
            for line in lines.iter() {
                result.push_str("- ");
                result.push_str(&line);
                result.push_str("\n\n");
            }
        }
    }

    // Technical
    result.push_str("## Technical\n");
    for (title, lines) in change_log.Technical.iter() {
        if !(lines.is_empty() || lines[0].is_empty()) {
            result.push_str("### ");
            result.push_str(&title);
            result.push_str("\n\n");
            for line in lines.iter() {
                result.push_str("- ");
                result.push_str(&line);
                result.push_str("\n\n");
            }
        }
    }

    result
}

//------------------------------------------------------------------------------
pub fn validate(path: &std::path::PathBuf) -> bool {
    // Make sure no invalid characters
    {
        use std::io::Read as _;
        let mut changelog_file = std::fs::File::open(
            path.to_str().expect("changelog path is not unicode"),
        )
        .expect(&format!(
            "Unable to read change log file '{}'",
            path.to_str().unwrap()
        ));

        let mut contents = String::new();
        changelog_file
            .read_to_string(&mut contents)
            .expect("Unable to read changelog for validation");

        // No tabs
        if contents.find("\t").is_some() {
            panic!("Changelog {:?} contains tabs", path);
        }

        // Make sure it can convert to ascii
        if !contents.is_ascii() {
            panic!("Changelog {:?} contains non ascii characters", path);
        }
    }

    // Make sure changelog structure is correct and not empty
    let change_log: Changelog = serde_yaml::from_reader(
        std::fs::File::open(
            path.to_str().expect("changelog path is not unicode"),
        )
        .expect(&format!(
            "Unable to read change log file '{}'",
            path.to_str().unwrap()
        )),
    )
    .expect(&format!(
        "Unable to parse the change log from disk '{}'",
        path.to_str().unwrap()
    ));

    // Make sure the change log isn't empty
    change_log.contains_entries()
}

//------------------------------------------------------------------------------
fn merge_work(lhs: &mut Work, rhs: &Work) {
    use heck::TitleCase as _;
    for (key, r) in rhs.iter() {
        if contains_something(r) {
            let key = key.to_title_case();
            match lhs.get_mut(&key) {
                Some(l) => {
                    l.extend_from_slice(&r[..]);
                }
                None => {
                    lhs.insert(key, r.clone());
                }
            }
        }
    }
}

//------------------------------------------------------------------------------
pub fn edit(name: &str, repo_path: Option<&std::path::Path>, commit: bool,
            msg_from_last_commit: Option<std::string::String>) {
    let changelog_path = resolve(name, repo_path);

    // If the changelog file doesnt exist then create a stub
    if !changelog_path.exists() {
        create_stub(name, repo_path);
    }

    // If a message is provided then put it under general
    if let Some(msg) = msg_from_last_commit {
        create_with_msg(name, repo_path, &msg);
    } else {
        std::process::Command::new("vi")
            .arg(&changelog_path)
            .spawn()
            .expect("failed to execute vi")
            .wait()
            .expect("failed to wait on vi");
    }

    if !validate(&changelog_path) {
        panic!("Changelog not valid");
    }

    std::process::Command::new("git")
        .arg("add")
        .arg(&changelog_path)
        .spawn()
        .expect("failed to execute git add")
        .wait()
        .expect("failed to wait on get add");

    if commit {
        let message = format!("update changelog for {}", name);
        std::process::Command::new("git")
            .arg("commit")
            .arg("-m")
            .arg(message)
            .spawn()
            .expect("failed to execute git add")
            .wait()
            .expect("failed to wait on get add");
    }
}

//------------------------------------------------------------------------------
pub fn aggregate(tag: &str, prefix: &[&str]) {
    // Obtain a list of all the changelog files that match the given prefixes.
    // These will be aggregated and combined into a single changelog.
    let mut change_logs: std::vec::Vec<std::path::PathBuf> =
        glob::glob(&format!("{}/**/*", &CHANGELOG))
            .expect("Failed to read glob")
            .filter(|e| {
                if let Ok(entry) = e {
                    if entry.is_file() {
                        let file_path = entry.to_str().unwrap();
                        for p in prefix.iter() {
                            if file_path[CHANGELOG.len() + 1..].starts_with(p) {
                                return true;
                            }
                        }
                    }
                }
                false
            })
            .map(|p| p.unwrap())
            .collect();

    change_logs.sort();
    let change_logs = change_logs;

    // Aggregate all the changelogs to produce a single one with the combined
    let mut aggregate_changelog = Changelog::empty();
    for changelog_file in change_logs.iter() {
        let changelog: Changelog = serde_yaml::from_reader(
            std::fs::File::open(&changelog_file).expect(&format!(
                "Unable to open changelog file '{}'",
                changelog_file.to_str().unwrap()
            )),
        )
        .expect(&format!(
            "Unable to read changelog file '{}'",
            changelog_file.to_str().unwrap()
        ));

        if changelog != Changelog::new() {
            // Combine all the artists notes
            if !changelog.Artists.is_empty() {
                merge_work(
                    &mut aggregate_changelog.Artists,
                    &changelog.Artists,
                );
            }

            // Combine all the technical notes
            if !changelog.Technical.is_empty() {
                merge_work(
                    &mut aggregate_changelog.Technical,
                    &changelog.Technical,
                );
            }
        }
    }

    if aggregate_changelog.is_empty() {
        println!(
            "Warning: There are no changes in the changelog for this release!"
        );
    }

    // Write the aggregate changelog to disk
    let aggregate_changelog_path = resolve(&format!("{}.e", tag), None);
    serde_yaml::to_writer(
        std::fs::File::create(&aggregate_changelog_path)
            .expect("Unable to create aggregate changelog file"),
        &aggregate_changelog,
    )
    .expect("Unable to write the aggregate changlog to disk");
}
