use assert_cmd::prelude::*;
use assert_fs::TempDir;
use assert_fs::prelude::*;
use predicates::prelude::*;
use std::process::Command;

pub fn temp_config_with(contents: &str) -> (TempDir, String) {
    let tmp = TempDir::new().expect("create temp dir");
    let cfg = tmp.child("config.toml");
    cfg.write_str(contents).expect("write config");
    (tmp, cfg.path().to_string_lossy().into_owned())
}

pub fn run_with_env(
    current_dir: &assert_fs::TempDir,
    envs: &[(&str, Option<&str>)],
) -> assert_cmd::assert::Assert {
    let mut cmd = Command::cargo_bin("feed_collect").expect("binary exists");
    cmd.current_dir(current_dir.path());

    for (k, v) in envs {
        match v {
            Some(val) => {
                cmd.env(k, val);
            }
            None => {
                cmd.env_remove(k);
            }
        }
    }

    cmd.assert()
}

pub fn extract_message_lines(stderr: &str) -> Vec<String> {
    stderr
        .lines()
        .filter_map(|line| {
            if let Some(idx) = line.find(" [") {
                // handle formats like "INFO [thread] message"
                let msg = &line[(idx + 1)..];
                // msg now starts with like "[thread] message"; find first occurrence of "[atom]" or "[tlgr]"
                if let Some(pos) = msg.find("[atom]").or_else(|| msg.find("[tlgr]")) {
                    return Some(msg[pos..].to_string());
                }
            }
            // Fallback: the whole line may already begin with message
            if line.contains("[atom]") || line.contains("[tlgr]") {
                Some(
                    line.split_once("[atom]")
                        .map(|(_, rest)| format!("[atom]{rest}"))
                        .or_else(|| {
                            line.split_once("[tlgr]")
                                .map(|(_, rest)| format!("[tlgr]{rest}"))
                        })
                        .unwrap(),
                )
            } else {
                None
            }
        })
        .collect()
}

pub fn assert_no_info_output(assert: &mut assert_cmd::assert::Assert) {
    use std::str;
    let output = assert.get_output();
    let stdout = str::from_utf8(&output.stdout).unwrap();
    let stderr = str::from_utf8(&output.stderr).unwrap();

    // We expect that info-level lines are not present
    let info_tag = predicate::str::contains("[atom]").or(predicate::str::contains("[tlgr]"));
    assert!(
        !info_tag.eval(stderr) && !info_tag.eval(stdout),
        "unexpected info output present: stdout=\n{}\nstderr=\n{}",
        stdout,
        stderr
    );
}
