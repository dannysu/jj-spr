/*
 * Copyright (c) Radical HQ Limited
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use crate::{
    error::{Error, Result},
    message::validate_commit_message,
    output::{output, write_commit_title},
};

#[derive(Debug, clap::Parser)]
pub struct FormatOptions {
    /// Format commits in range from base to revision
    #[clap(long, short = 'a')]
    all: bool,

    /// Base revision for --all mode (if not specified, uses trunk)
    #[clap(long)]
    base: Option<String>,
}

pub async fn format(
    opts: FormatOptions,
    jj: &crate::jj::Jujutsu,
    config: &crate::config::Config,
    revision: Option<&str>,
) -> Result<()> {
    let revision = revision.unwrap_or("@");
    
    let mut pc = if opts.all {
        let base = opts.base.as_deref().unwrap_or("trunk()");
        jj.get_prepared_commits_from_to(config, base, revision)?
    } else {
        vec![jj.get_prepared_commit_for_revision(config, revision)?]
    };

    if pc.is_empty() {
        output("👋", "No commits found - nothing to do. Good bye!")?;
        return Ok(());
    }

    let mut failure = false;

    for commit in pc.iter() {
        write_commit_title(commit)?;
        failure = validate_commit_message(&commit.message, config).is_err()
            || failure;
    }
    jj.rewrite_commit_messages(&mut pc)?;

    if failure {
        Err(Error::empty())
    } else {
        Ok(())
    }
}
