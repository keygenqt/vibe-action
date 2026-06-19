use crate::app_test_engine;

#[test]
fn test_list_all_false() {
    let output = app_test_engine("list-all-false");
    // No case matches and no fallback → should fail
    assert!(!output.status.success(), "Should fail when no case matches");
}

#[test]
fn test_list_all_true() {
    let output = app_test_engine("list-all-true");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    // All contain 'a' → case → upper
    assert!(stdout.contains("APPLE"), "Got: {}", stdout);
    assert!(stdout.contains("BANANA"), "Got: {}", stdout);
    assert!(stdout.contains("APRICOT"), "Got: {}", stdout);
    assert!(!stdout.contains("apple"), "Got: {}", stdout);
}

#[test]
fn test_list_is_file_all_false() {
    let output = app_test_engine("list-is-file-all-false");
    assert!(!output.status.success(), "Should fail when no files exist");
}

#[test]
fn test_list_is_file_mixed() {
    let output = app_test_engine("list-is-file-mixed");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    // Cargo.toml exists → case → only existing files
    assert!(stdout.contains("Cargo.toml"), "Got: {}", stdout);
    assert!(!stdout.contains("nonexistent.txt"), "Got: {}", stdout);
}

#[test]
fn test_list_mixed() {
    let output = app_test_engine("list-mixed");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    // apple, banana contain 'a' → case → upper (only these two)
    // cherry doesn't contain 'a' → filtered out
    assert!(stdout.contains("APPLE"), "Got: {}", stdout);
    assert!(stdout.contains("BANANA"), "Got: {}", stdout);
    assert!(!stdout.contains("CHERRY"), "Got: {}", stdout);
    assert!(!stdout.contains("cherry"), "Got: {}", stdout);
}

#[test]
fn test_string_case() {
    let output = app_test_engine("string-case");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("fixed"), "Got: {}", stdout);
}

#[test]
fn test_string_first_match() {
    let output = app_test_engine("string-first-match");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("first"), "Got: {}", stdout);
    assert!(!stdout.contains("second"), "Got: {}", stdout);
}

#[test]
fn test_string_no_match() {
    let output = app_test_engine("string-no-match");
    assert!(
        !output.status.success(),
        "Should fail when no case matches and no fallback"
    );
}

#[test]
fn test_mega_switch() {
    let output = app_test_engine("mega-switch");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Test 1: all contain 'a' → all upper (cherry filtered out)
    assert!(stdout.contains("APPLE"), "all_true failed: {}", stdout);
    assert!(stdout.contains("BANANA"), "all_true failed: {}", stdout);

    // Test 2: Cargo.toml exists → found; nonexistent.txt filtered out
    assert!(
        stdout.contains("found:Cargo.toml"),
        "mixed failed: {}",
        stdout
    );
    assert!(
        !stdout.contains("found:nonexistent.txt"),
        "mixed failed: {}",
        stdout
    );

    // Test 3: 2 users × 1 existing file = 2 combinations
    assert!(
        stdout.contains("user=alice file=Cargo.toml"),
        "multi_tag failed: {}",
        stdout
    );
    assert!(
        stdout.contains("user=bob file=Cargo.toml"),
        "multi_tag failed: {}",
        stdout
    );
    assert!(
        !stdout.contains("nonexistent.txt"),
        "multi_tag failed: {}",
        stdout
    );
}
