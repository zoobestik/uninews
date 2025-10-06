mod common;
use common::*;

fn make_minimal_config() -> String {
    // language=TOML
    r#"
[[atom]]
url = "https://example.com/über.xml"

[[telegram]]
nickname = "rust_lang_2024"
"#
    .to_string()
}

#[test]
fn snapshot_of_message_lines_is_stable() {
    let (tmp, cfg_path) = temp_config_with(&make_minimal_config());

    let assert = run_with_env(
        &tmp,
        &[
            ("MYFEED_CONFIG_PATH", Some(&cfg_path)),
            ("RUST_LOG", Some("info")),
        ],
    )
    .success();

    let stderr = String::from_utf8(assert.get_output().stderr.clone()).unwrap();
    let msgs = extract_message_lines(&stderr);

    // Stable order expected: atoms first, then telegrams
    let joined = msgs.join("\n");

    insta::assert_snapshot!(&joined, @r###"[atom] https://example.com/%C3%BCber.xml
[tlgr] https://t.me/rust_lang_2024"###);
}
