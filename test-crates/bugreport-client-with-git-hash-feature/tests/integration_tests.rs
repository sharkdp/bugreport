use bugreport_test_crate_utils::assert_bin_stdout;

#[test]
fn with_git_repo() {
    assert_bin_stdout(
        "bugreport-client-with-git-hash-feature",
        r"#### Software version

bugreport-client-with-git-hash-feature 0\.1\.0 \([0-9a-f]{7,}(-modified)?\)


",
    );
}

// NOTE: You must only run this test with GIT_DIR set to
// something at build time that is not a git repo
#[test]
fn without_git_repo() {
    assert_bin_stdout(
        "bugreport-client-with-git-hash-feature",
        r"#### Software version

bugreport-client-with-git-hash-feature 0\.1\.0


",
    );
}
