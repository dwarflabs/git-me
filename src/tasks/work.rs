//------------------------------------------------------------------------------
// from+git_me@luketitley.com
//------------------------------------------------------------------------------
use crate::branch;
use crate::changelog;
use crate::server;
use crate::teams;

//------------------------------------------------------------------------------
pub fn start(branch_type: branch::Type, name: &str) {
    // Verify there's nothing in the index
    println!("    * Check nothing to commit");
    if !branch::verify_index_empty() {
        panic!("You have uncommited changes, please stash them.");
    }

    // Verify the branch name has valid chars in it
    println!("    * Check name '{}' is well formed", name);
    if !branch::well_formed(name) {
        panic!("Your branch name has invalid characters in it '{}'.", name);
    }

    // Find the user specified in reviewer
    let mut server = server::Server::new();

    // Make the new branch
    println!("    * {}", &branch::resolve(branch_type, name));
    branch::branch(branch_type, name);

    // Push the new branch
    println!("    * push");
    branch::push(&branch::resolve(branch_type, name));

    // Set upstream
    println!("    * set upstream");
    branch::set_upstream(&branch::resolve(branch_type, name));

    // Create a new merge request upfront
    println!("    * wip merge request");
    let remote_url = branch::find_remote();
    let project = server.project(&remote_url);
    server.merge_request(
        &project,
        branch::base(branch_type),
        &branch::resolve(branch_type, name),
    );
}

//------------------------------------------------------------------------------
pub fn review(finished: std::vec::Vec<std::string::String>) {
    let branch_type = if let Some(branch_type) = branch::find_type() {
        branch_type
    } else {
        panic!("Unable to determine branch type, must be hotfix or feature");
    };

    let branch_name = branch::find_name();
    let repo_path = branch::find_path();

    // Verify there's nothing in the index
    println!("    * Check outstanding changes");
    if !branch::verify_index_empty() {
        panic!("You have uncommited changes");
    }

    // Verify that our branch is up to speed
    println!("    * Check rebased");
    let mut server = server::Server::new();
    let remote_url = branch::find_remote();
    let head_commit =
        server.find_head_commit(&remote_url, branch::base(branch_type));

    // Verify that your branch is rebased on top of the latest work in base
    if !branch::verify_up_to_date(&head_commit, &branch_name) {
        println!("        * Rebasing");
        branch::rebase_place_holder(branch_type, &branch_name);
    }

    // Push your work
    println!("    * Push");
    branch::push(&branch_name);

    // Remove the wip status
    let reviewers = finished;
    if !reviewers.is_empty() {
        // Were finished
        println!("    * Finished");

        // Verify the changelog has been filled out
        println!("        * Check changelog");
        if !changelog::verify(&branch_name, Some(&repo_path)) {
            panic!("You've not filled in your changelog");
        }

        let mut primary_reviewer = 0_u64;

        // Check reviewers are valid
        let mut reviewers_names = std::vec::Vec::new();
        for (r_index, r) in reviewers.iter().enumerate() {
            println!("        * Check {} exists", r);
            let user = server.find_user(&r, true);
            reviewers_names.push(user.name);
            if r_index == 0 {
                primary_reviewer = user.id;
            }
        }

        // Get developer name
        let dev_name = std::env::var("USER")
            .expect("Unable to find developer name locally");
        let dev_name = server.find_user(&dev_name, false).name;

        // Get changelog
        let changelog = changelog::read_formatted(&changelog::resolve(
            &branch_name,
            Some(&repo_path),
        ));

        // Remove WIP
        println!("        * Remove WIP");
        let project = server.project(&remote_url);
        let [title, summary] = server.final_merge_request(
            &project,
            &branch_name,
            &changelog,
            &reviewers_names,
            primary_reviewer,
            &dev_name
        );

        // Send the merge request
        println!("        * Sending MR to teams");
        teams::send_mr(&title, &summary);

        //println!("\n\n{}", summary);
    }
}

//------------------------------------------------------------------------------
pub fn rebase(branch_type: branch::Type) {
    branch::rebase_place_holder(branch_type, &branch::find_name());
}
