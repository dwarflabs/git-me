//------------------------------------------------------------------------------
// from+git_me@luketitley.com
//------------------------------------------------------------------------------
use crate::branch;
use crate::changelog;

//------------------------------------------------------------------------------
pub fn aggregate(tag: &str) {
    // Build the aggregate changelog
    let _ = changelog::aggregate(tag, &["feature", "hotfix"]);
}

//------------------------------------------------------------------------------
pub fn validate(path: &str) {
    // Build the aggregate changelog
    if !changelog::validate(&std::path::PathBuf::from(path)) {
        panic!("Failed to validate {}", path);
    }
}

//------------------------------------------------------------------------------
pub fn edit(commit: bool, from_last_commit: bool) {
    let branch_name = branch::find_name();

    let msg = if from_last_commit {
        let last_commit_msg = branch::find_last_commit_msg()
                                            .replace('\n', "");
        Some(last_commit_msg)
    } else {
        None
    };

    if ! (branch_name.starts_with("hotfix") ||
            branch_name.starts_with("feature")) {
        panic!("You are on the {} branch. You can only edit changelogs on feature or hotfix branches", branch_name);
    }

    let repo_path = branch::find_path();

    changelog::edit(&branch_name, Some(&repo_path), commit, msg);
}
