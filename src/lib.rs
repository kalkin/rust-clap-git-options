//! Provides the iconic `-C`, `--git-dir` & `--work-tree` git options

use clap::ValueHint;

/// The `clap::Args` struct
#[derive(clap::Args, Default, Debug)]
#[clap(next_help_heading = "Git Options")]
pub struct GitOptions {
    /// Run as if was started in <path>
    #[clap(short = 'C', num_args = 1, value_hint=ValueHint::DirPath)]
    pub change_dir: Option<String>,
    /// Directory where the GIT_DIR is
    #[clap(long, value_hint=ValueHint::DirPath)]
    pub git_dir: Option<String>,
    /// Directory where the GIT_WORK_TREE is
    #[clap(long, value_hint=ValueHint::DirPath)]
    pub work_tree: Option<String>,
}

#[cfg(feature = "git-wrapper")]
impl TryFrom<&GitOptions> for git_wrapper::Repository {
    type Error = git_wrapper::RepoError;
    #[inline]
    fn try_from(args: &GitOptions) -> Result<Self, Self::Error> {
        Self::from_args(
            args.change_dir.as_deref(),
            args.git_dir.as_deref(),
            args.work_tree.as_deref(),
        )
    }
}
