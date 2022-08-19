//------------------------------------------------------------------------------
// from+git_me@luketitley.com
//------------------------------------------------------------------------------
use argh::FromArgs;

//------------------------------------------------------------------------------
#[derive(FromArgs, PartialEq, Debug)]
/// A tool to optimise development workflow.
///
/// Typical workflow:
/// - git feature start -n my_cool_feature                                   # Start a new feature branch
///  - ...                                                                    # Your normal git commands
///  - git changelog edit --commit/-c                                         # Edit your changelog in vim
///  - git feature review                                                     # Dry-run, check you're ready to review
///  - git feature review --finished/-f <your reviewer username in gitlab>    # Your done,
///
/// When you run git feature review, that will remove WIP from your MR and
/// send a message to the teams merge request channel.
//
pub struct Tasks {
    #[argh(subcommand)]
    pub task: Task,
}

//------------------------------------------------------------------------------
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
pub enum Task {
    Feature(Feature),
    Hotfix(Hotfix),
    Review(Review),
    Changelog(Changelog),
    Setup(Setup),
    Info(Info),
    Project(Project),
}

//------------------------------------------------------------------------------
#[derive(FromArgs, PartialEq, Debug)]
/// start
#[argh(subcommand, name = "start")]
pub struct Start {
    #[argh(option, short = 'n')]
    /// name of the new feature/hotfix
    pub name: std::string::String,
}

//------------------------------------------------------------------------------
#[derive(FromArgs, PartialEq, Debug)]
/// review
#[argh(subcommand, name = "review")]
pub struct Review {
    #[argh(option, short = 'f')]
    /// is this the final code review?
    pub finished: std::vec::Vec<std::string::String>,
}

//------------------------------------------------------------------------------
#[derive(FromArgs, PartialEq, Debug)]
/// rebase
#[argh(subcommand, name = "rebase")]
pub struct Rebase {}

//------------------------------------------------------------------------------
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
pub enum Status {
    Start(Start),
    Rebase(Rebase),
}

//------------------------------------------------------------------------------
#[derive(FromArgs, PartialEq, Debug)]
/// Working with feature
#[argh(subcommand, name = "feature")]
pub struct Feature {
    #[argh(subcommand)]
    /// the stage in the feature
    pub status: Status,
}

//------------------------------------------------------------------------------
#[derive(FromArgs, PartialEq, Debug)]
/// Working with hotfix
#[argh(subcommand, name = "hotfix")]
pub struct Hotfix {
    #[argh(subcommand)]
    /// the stage in the feature
    pub status: Status,
}

//------------------------------------------------------------------------------
#[derive(FromArgs, PartialEq, Debug)]
/// Build a changelog for a version, by merging feature changelogs
#[argh(subcommand, name = "aggregate")]
pub struct Aggregate {
    #[argh(option)]
    /// the tag version we will use for this changelog
    pub tag: std::string::String,
}

//------------------------------------------------------------------------------
#[derive(FromArgs, PartialEq, Debug)]
/// Test that the changelog is valid
#[argh(subcommand, name = "validate")]
pub struct Validate {
    #[argh(option)]
    /// the tag version we will use for this changelog
    pub path: std::string::String,
}

//------------------------------------------------------------------------------
#[derive(FromArgs, PartialEq, Debug)]
/// Edit the changelog directly with vi
#[argh(subcommand, name = "edit")]
pub struct Edit {
    #[argh(switch, short = 'c')]
    /// commit the changelog after its been changed
    pub commit: bool,
    #[argh(switch, short = 'l')]
    /// the changelog message should be taken from the last commit
    pub last_commit: bool,
}

//------------------------------------------------------------------------------
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
pub enum ChangelogStatus {
    Aggregate(Aggregate),
    Validate(Validate),
    Edit(Edit),
}

//------------------------------------------------------------------------------
#[derive(FromArgs, PartialEq, Debug)]
/// Operations for working with changelogs
#[argh(subcommand, name = "changelog")]
pub struct Changelog {
    #[argh(subcommand)]
    /// combine the feature changlogs
    pub status: ChangelogStatus,
}

//------------------------------------------------------------------------------
#[derive(FromArgs, PartialEq, Debug)]
/// List the projects
#[argh(subcommand, name = "list")]
pub struct ProjectList {}

//------------------------------------------------------------------------------
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
pub enum ProjectCommand {
    List(ProjectList),
}

//------------------------------------------------------------------------------
#[derive(FromArgs, PartialEq, Debug)]
/// The initial setup of git-me
#[argh(subcommand, name = "project")]
pub struct Project {
    #[argh(subcommand)]
    /// what do we want to do with projects
    pub project: ProjectCommand,
}

//------------------------------------------------------------------------------
#[derive(FromArgs, PartialEq, Debug)]
/// The initial setup of git-me
#[argh(subcommand, name = "setup")]
pub struct Setup {
    #[argh(option)]
    /// the gitlab server
    pub server: std::string::String,
    #[argh(option)]
    /// api token
    pub private_token: std::string::String,
}

//------------------------------------------------------------------------------
#[derive(FromArgs, PartialEq, Debug)]
/// The initial setup of git-me
#[argh(subcommand, name = "info")]
pub struct Info {}
