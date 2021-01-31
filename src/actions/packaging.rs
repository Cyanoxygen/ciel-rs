use anyhow::{anyhow, Result};
use chrono::Duration;
use console::{style, Term};
use std::{
    fs,
    io::{BufRead, BufReader},
    path::Path,
    time::Instant,
};

use crate::{config, error, info, repo, warn};

use super::{UPDATE_SCRIPT, container::{get_output_directory, mount_fs, rollback_container, run_in_container}, stop_container};

#[inline]
fn format_duration(duration: Duration) -> String {
    let seconds = duration.num_seconds();
    format!(
        "{:02}:{:02}:{:02}",
        seconds / 3600,
        (seconds / 60) % 60,
        seconds % 60
    )
}

fn read_package_list<P: AsRef<Path>>(filename: P) -> Result<Vec<String>> {
    let f = fs::File::open(filename)?;
    let reader = BufReader::new(f);
    let mut results = Vec::new();
    for line in reader.lines() {
        let line = line?;
        // skip comment
        if line.starts_with('#') {
            continue;
        }
        // trim whitespace
        let trimmed = line.trim();
        results.push(trimmed.to_owned());
    }

    Ok(results)
}

fn expand_package_list<'a, I: IntoIterator<Item = &'a str>>(packages: I) -> Vec<String> {
    let mut expanded = Vec::new();
    for package in packages {
        if !package.starts_with("groups/") {
            expanded.push(package.to_string());
            continue;
        }
        let list_file = Path::new("./TREE").join(&package);
        match read_package_list(list_file) {
            Ok(list) => {
                info!("Read {} packages from {}", list.len(), package);
                expanded.extend(list);
            }
            Err(e) => {
                warn!("Unable to read package group `{}`: {}", package, e);
            }
        }
    }

    expanded
}

/// Build packages in the container
pub fn package_build<'a, K: ExactSizeIterator<Item = &'a str>>(
    instance: &str,
    packages: K,
) -> Result<i32> {
    let conf = config::read_config();
    if conf.is_err() {
        return Err(anyhow!("Please configure this workspace first!"));
    }
    let conf = conf.unwrap();

    stop_container(instance)?;

    if !conf.local_repo {
        let mut cmd = vec!["/bin/acbs-build", "--"];
        cmd.extend(packages.into_iter());
        let status = run_in_container(instance, &cmd)?;
        return Ok(status);
    }

    let output_dir = get_output_directory(conf.sep_mount);
    let root = std::env::current_dir()?.join(output_dir);
    let term = Term::stderr();
    let packages = expand_package_list(packages);
    let total = packages.len();
    let start = Instant::now();
    for (index, package) in packages.into_iter().enumerate() {
        info!("[{}/{}] Building {}...", index + 1, total, package);
        mount_fs(&instance)?;
        info!("Refreshing local repository...");
        repo::init_repo(&root, Path::new(instance))?;
        let status = run_in_container(&instance, &["/bin/bash", "-ec", UPDATE_SCRIPT])?;
        if status != 0 {
            error!("Failed to update the OS before building packages");
            return Ok(status);
        }
        term.set_title(format!("ciel: [{}/{}] {}", index + 1, total, package));
        term.flush().ok();
        let status = run_in_container(instance, &["/bin/acbs-build", "--", &package])?;
        if status != 0 {
            error!("Build failed with status: {}", status);
            return Ok(status);
        }
        rollback_container(instance)?;
    }
    let duration = Duration::from_std(start.elapsed())?;
    eprintln!(
        "{} - {} packages in {}",
        style("BUILD SUCCESSFUL").bold().green(),
        total,
        format_duration(duration)
    );
    // clear terminal title
    term.set_title("");

    Ok(0)
}

#[test]
fn test_time_format() {
    let test_dur = Duration::seconds(3661);
    assert_eq!(format_duration(test_dur), "01:01:01");
}