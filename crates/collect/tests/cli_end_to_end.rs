mod common;
use assert_fs::prelude::*;
use common::*;
use predicates::prelude::*;

// Helper to compose a valid config string
fn sample_config() -> String {
    let toml = r#"
[[atom]]
url = "https://a.com/1"
[[atom]]
url = "https://a.com/2"

[[telegram]]
nickname = "rustaceans"
[[telegram]]
nickname = "rust_lang_2024"
"#;
    toml.to_string()
}

#[test]
fn outputs_one_line_per_source_with_order_and_format() {
    let (tmp, cfg_path) = temp_config_with(&sample_config());

    // Ensure we get info-level logs
    let mut assert = run_with_env(
        &tmp,
        &[
            ("MYFEED_CONFIG_PATH", Some(&cfg_path)),
            ("RUST_LOG", Some("info")),
        ],
    )
    .success()
    .stdout(predicate::str::is_empty()) // do not mix logs into stdout
    .stderr(predicate::str::contains("[atom] https://a.com/1"))
    .stderr(predicate::str::contains("[atom] https://a.com/2"))
    .stderr(predicate::str::contains("[tlgr] https://t.me/rustaceans"))
    .stderr(predicate::str::contains(
        "[tlgr] https://t.me/rust_lang_2024",
    ));

    // Extract message-only lines and assert exact order
    let out = String::from_utf8(assert.get_output().stderr.clone()).unwrap();
    let msgs = extract_message_lines(&out);

    let expected = [
        "[atom] https://a.com/1".to_string(),
        "[atom] https://a.com/2".to_string(),
        "[tlgr] https://t.me/rustaceans".to_string(),
        "[tlgr] https://t.me/rust_lang_2024".to_string(),
    ];

    // Find subsequence in order (allowing potential non-related logs)
    let mut idx = 0usize;
    for m in msgs {
        if idx < expected.len() && m == expected[idx] {
            idx += 1;
        }
    }
    assert_eq!(idx, expected.len(), "messages should appear in order");
}

#[test]
fn default_config_path_used_when_env_unset() {
    let tmp = assert_fs::TempDir::new().unwrap();
    // Write default ./config.toml inside temp dir
    tmp.child("config.toml")
        .write_str(&sample_config())
        .unwrap();

    let mut assert = run_with_env(
        &tmp,
        &[("MYFEED_CONFIG_PATH", None), ("RUST_LOG", Some("info"))],
    )
    .success()
    .stdout(predicate::str::is_empty())
    .stderr(predicate::str::contains("[atom] https://a.com/1"));

    let out = String::from_utf8(assert.get_output().stderr.clone()).unwrap();
    let msgs = extract_message_lines(&out);
    assert!(msgs.iter().any(|m| m == "[tlgr] https://t.me/rustaceans"));
}

#[test]
fn missing_file_produces_error_and_exit_code_1() {
    let tmp = assert_fs::TempDir::new().unwrap();

    run_with_env(&tmp, &[("MYFEED_CONFIG_PATH", Some("./no_such.toml"))])
        .failure()
        .code(1)
        .stderr(predicate::str::contains("Failed to read config file"));
}

#[test]
fn invalid_toml_produces_error_and_exit_code_1() {
    let tmp = assert_fs::TempDir::new().unwrap();
    let bad = "[telegram\n nickname = 'abc'"; // malformed
    tmp.child("bad.toml").write_str(bad).unwrap();

    run_with_env(&tmp, &[("MYFEED_CONFIG_PATH", Some("./bad.toml"))])
        .failure()
        .code(1)
        .stderr(predicate::str::contains("Failed to read config file"))
        .stderr(predicate::str::contains("Failed to parse TOML"));
}

#[cfg(unix)]
#[test]
fn unreadable_file_results_in_error() {
    use std::fs;
    use std::os::unix::fs::PermissionsExt;

    let tmp = assert_fs::TempDir::new().unwrap();
    let p = tmp.child("secret.toml");
    p.write_str(&sample_config()).unwrap();

    // remove all permissions
    let mut perms = fs::metadata(p.path()).unwrap().permissions();
    perms.set_mode(0o000);
    fs::set_permissions(p.path(), perms).unwrap();

    run_with_env(&tmp, &[("MYFEED_CONFIG_PATH", Some("./secret.toml"))])
        .failure()
        .code(1)
        .stderr(predicate::str::contains("Failed to read config file"));
}
