//------------------------------------------------------------------------------
// from+git_me@luketitley.com
//------------------------------------------------------------------------------
use crate::config;

#[derive(
    Debug, PartialEq, serde::Serialize, serde::Deserialize, Default, Clone,
)]
pub struct Project {
    pub id: u64,
    pub path_with_namespace: std::string::String,
    pub ssh_url_to_repo: std::string::String,
}

#[derive(
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Clone,
    Eq,
    PartialOrd,
)]
pub struct User {
    pub username: std::string::String,
    pub name: std::string::String,
    pub id: u64,
}

#[derive(
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Clone,
    Eq,
    PartialOrd,
)]
pub struct Branch {
    pub name: std::string::String,
    pub commit: Commit,
}

#[derive(
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    Clone,
    Eq,
)]
pub struct MergeRequest {
    pub iid: u64,
    pub web_url: std::string::String,
    pub state: gitlab::types::MergeRequestState,
}

impl Default for MergeRequest {
    fn default() -> Self {
        MergeRequest {
            iid: 0_u64,
            web_url: "".to_string(),
            state: gitlab::types::MergeRequestState::Merged,
        }
    }
}

impl PartialOrd for MergeRequest {
    fn partial_cmp(&self, other: &MergeRequest) -> Option<std::cmp::Ordering> {
        self.iid.partial_cmp(&other.iid)
    }
}

#[derive(
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Clone,
    Eq,
    PartialOrd,
)]
pub struct Commit {
    pub id: std::string::String,
}

impl std::cmp::Ord for User {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.username.cmp(&other.username)
    }
}

pub struct Server {
    server: gitlab::Gitlab,
}

pub fn from_nick(name: &str) -> &str {
    name
}

pub fn to_nick(name: &str) -> &str {
    name
}

impl Server {
    pub fn new() -> Self {
        let config = config::Config::open();
        Server {
            server: gitlab::Gitlab::new(&config.server, &config.private_token)
                .expect("Unable to connect to server"),
        }
    }

    pub fn project(&self, url: &str) -> Project {
        let pageable_endpoint = gitlab::api::projects::Projects::builder()
            .build()
            .expect("Unable to list all the project in the gitlab server");

        use gitlab::api::Query as _;
        let projects: Vec<Project> =
            gitlab::api::paged(pageable_endpoint, gitlab::api::Pagination::All)
                .query(&self.server)
                .expect("List projects query failed");

        for project in projects.iter() {
            if &project.ssh_url_to_repo == url {
                return project.clone();
            }
        }

        println!("Unable to find gitlab project for current repo {}", url);
        println!("Projects are:");
        for project in projects.iter() {
            println!("    {}", &project.ssh_url_to_repo);
        }

        println!("It could be that your repo url is out of date. Try:");
        println!("    git remote remove origin");
        println!("    git remote add origin <<your new url goes here>>");

        panic!("See error above");
    }

    pub fn find_user(&mut self, mut username: &str, resolve_nick: bool) -> User {
        if resolve_nick {
            username = from_nick(username);
        }

        let pageable_endpoint = gitlab::api::users::Users::builder()
            .build()
            .expect("Unable to list all the users in the gitlab server");

        use gitlab::api::Query as _;
        let mut users: Vec<User> =
            gitlab::api::paged(pageable_endpoint, gitlab::api::Pagination::All)
                .query(&self.server)
                .expect("List users query failed");

        for user in users.iter() {
            if &user.username == username {
                return user.clone();
            }
        }

        println!("Unable to find user '{}' users are:", username);
        users.sort();
        for user in users.iter() {
            let nick = to_nick(&user.username);
            println!("    {}", nick);
        }

        panic!("Unable to find user")
    }

    pub fn find_head_commit(
        &mut self,
        url: &str,
        name: &str,
    ) -> std::string::String {
        let project = self.project(url);

        let endpoint =
            gitlab::api::projects::repository::branches::Branches::builder()
                .project(project.id)
                .build()
                .expect("Unable to list all the branches in the given project");

        use gitlab::api::Query as _;
        let branches: Vec<Branch> = endpoint
            .query(&self.server)
            .expect("List projects query failed");

        for branch in branches.iter() {
            if &branch.name == name {
                return branch.commit.id.clone();
            }
        }

        panic!("Given branch {} not found", name);
    }

    pub fn merge_request(
        &mut self,
        project: &Project,
        base: &str,
        branch: &str,
    ) {
        let title = format!("WIP: {}", branch);
        let endpoint =
            gitlab::api::projects::merge_requests::CreateMergeRequest::builder(
            )
            .project(project.id)
            .source_branch(branch)
            .remove_source_branch(true)
            .target_branch(base)
            .title(&title)
            .build()
            .expect("Unable to list all the project in the gitlab server");

        use gitlab::api::Query as _;
        gitlab::api::ignore(endpoint)
            .query(&self.server)
            .expect("Create merge request failed");
    }

    pub fn find_merge_request(
        &self,
        project: &Project,
        branch: &str,
    ) -> Option<(u64, std::string::String, gitlab::types::MergeRequestState)> {
        let endpoint =
            gitlab::api::projects::merge_requests::MergeRequests::builder()
                .project(project.id)
                .source_branch(branch)
                .build()
                .expect("Unable to find merge request");

        use gitlab::api::Query as _;
        let mrs: Vec<MergeRequest> = endpoint
            .query(&self.server)
            .expect("List merge request query failed");

        if !mrs.is_empty() {
            Some((mrs[0].iid, mrs[0].web_url.clone(), mrs[0].state))
        } else {
            None
        }
    }

    pub fn list_projects<F>(&self, mut f: F)
    where
        F: FnMut(&Project),
    {
        let pageable_endpoint = gitlab::api::projects::Projects::builder()
            .build()
            .expect("Unable to list all the project in the gitlab server");

        use gitlab::api::Query as _;
        let projects: Vec<Project> =
            gitlab::api::paged(pageable_endpoint, gitlab::api::Pagination::All)
                .query(&self.server)
                .expect("List projects query failed");

        for project in projects.iter() {
            f(project)
        }
    }

    pub fn final_merge_request(
        &self,
        project: &Project,
        branch: &str,
        description: &str,
        reviewers: &[std::string::String],
        primary_reviewer: u64,
        dev_name: &str,
    ) -> [std::string::String; 2] {
        let new_title = format!("{}", branch);
        let (merge_request_id, merge_request_url, merge_request_state) = self
            .find_merge_request(project, branch)
            .expect("Unable to find merge request");

        let endpoint =
            gitlab::api::projects::merge_requests::EditMergeRequest::builder()
                .project(project.id)
                .merge_request(merge_request_id)
                .title(&new_title)
                .assignee(primary_reviewer)
                .description(description)
                .state_event(gitlab::api::projects::merge_requests::MergeRequestStateEvent::Reopen)
                .build()
                .expect("Unable to edit merge request");

        use gitlab::api::Query as _;
        gitlab::api::ignore(endpoint)
            .query(&self.server)
            .expect("Edit merge request failed");

        let mut mention = std::string::String::new();
        for r in reviewers.iter() {
            mention.push_str(&format!("<at>{}</at>\n\n", r));
        }

        [
            branch.to_string(),
            format!(
                r#"
{}  

{}  

_{}_  

---

{}
"#,
                mention, merge_request_url, dev_name, description
            ),
        ]
    }
}
