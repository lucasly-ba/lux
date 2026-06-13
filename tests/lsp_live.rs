//! A *live* test of the LSP client against a real `rust-analyzer`.
//!
//! This is `#[ignore]`d because it spawns rust-analyzer and waits for it to
//! analyse a crate, which is slow and needs the binary installed. Run it
//! explicitly to validate the end-to-end language-server path:
//!
//! ```sh
//! cargo test --test lsp_live -- --ignored --nocapture
//! ```

use std::time::{Duration, Instant};

use lux::lsp::LspClient;
use lux::lsp::protocol::path_to_uri;

#[test]
#[ignore = "spawns rust-analyzer; run with --ignored"]
fn rust_analyzer_publishes_a_diagnostic() {
    // Build a throwaway cargo crate containing an obvious type error.
    let dir = std::env::temp_dir().join(format!("lux-lsp-live-{}", std::process::id()));
    let src = dir.join("src");
    std::fs::create_dir_all(&src).unwrap();
    std::fs::write(
        dir.join("Cargo.toml"),
        "[package]\nname = \"luxlsptest\"\nversion = \"0.1.0\"\nedition = \"2021\"\n",
    )
    .unwrap();
    let main = src.join("main.rs");
    // `let x: i32 = "oops";` is a type error rust-analyzer flags from its own
    // inference, without needing a full `cargo check`.
    std::fs::write(
        &main,
        "fn main() {\n    let x: i32 = \"oops\";\n    let _ = x;\n}\n",
    )
    .unwrap();

    let abs = std::fs::canonicalize(&main).unwrap();
    let uri = path_to_uri(&abs);

    let mut client = LspClient::start("rust-analyzer", &dir).expect("rust-analyzer should start");
    // did_open(uri, language_id, version, text)
    client
        .did_open(&uri, "rust", 0, &std::fs::read_to_string(&abs).unwrap())
        .unwrap();

    // Poll for up to a minute; this also proves poll_diagnostics doesn't hang on
    // the stream of $/progress notifications rust-analyzer sends while indexing.
    let deadline = Instant::now() + Duration::from_secs(60);
    let mut found = false;
    while Instant::now() < deadline {
        if let Some((doc_uri, diags)) = client.poll_diagnostics() {
            eprintln!("diagnostics for {doc_uri}: {} item(s)", diags.len());
            if doc_uri == uri && !diags.is_empty() {
                eprintln!("  -> {}", diags[0].message);
                found = true;
                break;
            }
        }
        std::thread::sleep(Duration::from_millis(100));
    }

    let _ = std::fs::remove_dir_all(&dir);
    assert!(
        found,
        "expected rust-analyzer to publish a diagnostic within 60s"
    );
}
