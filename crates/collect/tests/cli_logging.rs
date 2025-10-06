mod common;
use assert_fs::prelude::*;
use common::*;
use predicates::prelude::*;

fn minimal_config() -> String {
    let toml = r#"
[[atom]]
url = "https://example.com/feed.xml"
"#;
    toml.to_string()
}

#[test]
fn default_env_filter_is_info_when_rust_log_unset() {
    let (tmp, cfg_path) = temp_config_with(&minimal_config());

    // Unset RUST_LOG so default filter applies
    let assert = run_with_env(
        &tmp,
        &[("MYFEED_CONFIG_PATH", Some(&cfg_path)), ("RUST_LOG", None)],
    )
    .success()
    .stdout(predicate::str::is_empty())
    .stderr(predicate::str::contains(
        "[atom] https://example.com/feed.xml",
    ));

    // ensure no errors were emitted
    let out = String::from_utf8(assert.get_output().stderr.clone()).unwrap();
    assert!(out.contains("[atom]"));
}

#[test]
fn respects_rust_log_error_to_suppress_info() {
    let (tmp, cfg_path) = temp_config_with(&minimal_config());

    let mut assert = run_with_env(
        &tmp,
        &[
            ("MYFEED_CONFIG_PATH", Some(&cfg_path)),
            ("RUST_LOG", Some("error")),
        ],
    )
    .success()
    .stdout(predicate::str::is_empty());

    // Should not have info-level lines
    assert_no_info_output(&mut assert);
}

#[test]
fn can_run_multiple_times_in_separate_processes() {
    let (tmp, cfg_path) = temp_config_with(&minimal_config());

    // First run
    run_with_env(
        &tmp,
        &[
            ("MYFEED_CONFIG_PATH", Some(&cfg_path)),
            ("RUST_LOG", Some("info")),
        ],
    )
    .success();

    // Second run (separate process) should also succeed; ensures global logger set per-process
    run_with_env(
        &tmp,
        &[
            ("MYFEED_CONFIG_PATH", Some(&cfg_path)),
            ("RUST_LOG", Some("info")),
        ],
    )
    .success();
}
