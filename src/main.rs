//------------------------------------------------------------------------------
// from+git_me@luketitley.com
//------------------------------------------------------------------------------
mod args;
mod branch;
mod changelog;
mod config;
mod server;
mod tasks;
mod teams;

use args::*;

//------------------------------------------------------------------------------
fn main() {
    let tasks: Tasks = argh::from_env();

    match tasks.task {
        // Feature
        Task::Feature(Feature { status }) => match status {
            Status::Start(Start { name }) => {
                tasks::work::start(branch::Type::Feature, &name)
            }
            Status::Rebase(Rebase {}) => {
                println!("wip");
                //println!("try: 'git checkout '");
                //tasks::work::rebase(branch::Type::Feature)
            }
        },
        // Hotfix
        Task::Hotfix(Hotfix { status }) => match status {
            Status::Start(Start { name }) => {
                tasks::work::start(branch::Type::Hotfix, &name)
            }
            Status::Rebase(Rebase {}) => {
                println!("wip");
                //tasks::work::rebase(branch::Type::Hotfix)
            }
        },
        Task::Review(Review { finished }) => tasks::work::review(finished),
        Task::Changelog(Changelog {
            status: ChangelogStatus::Aggregate(Aggregate { tag }),
        }) => {
            tasks::changelog::aggregate(&tag);
        }
        Task::Changelog(Changelog {
            status: ChangelogStatus::Validate(Validate { path }),
        }) => {
            tasks::changelog::validate(&path);
        }
        Task::Changelog(Changelog {
            status: ChangelogStatus::Edit(Edit { commit, last_commit }),
        }) => {
            tasks::changelog::edit(commit, last_commit);
        }
        Task::Setup(Setup {
            server,
            private_token,
        }) => {
            tasks::setup::setup(&server, &private_token);
        }
        Task::Info(Info {}) => {
            tasks::setup::info();
        }
        Task::Project(Project {
            project: ProjectCommand::List(list),
        }) => {
            let sv = server::Server::new();
            sv.list_projects(|project| println!("{}", project.ssh_url_to_repo));
        }
        _ => (),
    }
}
