use rustsec::{advisory, package, Database, Lockfile, Repository, VersionReq, DEFAULT_REPO_URL};
use tempfile::tempdir;

/// Happy path integration test (has online dependency on GitHub)
#[test]
fn happy_path() {
    let db = Database::fetch().unwrap();
    verify_rustsec_2017_0001(&db);
    verify_cve_2018_1000810(&db);
}

/// End-to-end integration test (has online dependency on GitHub) which looks
/// for the `RUSTSEC-2017-0001` vulnerability (sodiumoxide crate).
fn verify_rustsec_2017_0001(db: &Database) {
    let example_advisory_id = "RUSTSEC-2017-0001".parse::<advisory::Id>().unwrap();
    let example_advisory = db.find(&example_advisory_id).unwrap();
    let example_package = package::Name::from("sodiumoxide");

    assert_eq!(example_advisory.metadata.id, example_advisory_id);
    assert_eq!(example_advisory.metadata.package, example_package);
    assert_eq!(
        example_advisory.versions.patched[0],
        VersionReq::parse(">= 0.0.14").unwrap()
    );
    assert_eq!(example_advisory.metadata.date.as_str(), "2017-01-26");
    assert_eq!(
        example_advisory.metadata.url.as_ref().unwrap(),
        "https://github.com/dnaq/sodiumoxide/issues/154"
    );
    assert_eq!(
        example_advisory.metadata.title,
        "scalarmult() vulnerable to degenerate public keys"
    );
    assert_eq!(
        &example_advisory.metadata.description[0..30],
        "The `scalarmult()` function in"
    );
    assert_eq!(
        example_advisory.metadata.collection.unwrap(),
        package::Collection::Crates
    );

    let ref crate_advisories = db.find_by_crate(example_package);
    assert_eq!(example_advisory, crate_advisories[0]);

    let lockfile = Lockfile::load_file("Cargo.lock").unwrap();
    let vulns = db.vulnerabilities(&lockfile);
    assert!(vulns.is_empty());
}

/// End-to-end integration test (has online dependency on GitHub) which looks
/// for the `CVE-2018-1000810` vulnerability (`std::str::repeat`)
fn verify_cve_2018_1000810(db: &Database) {
    let example_advisory_id = "CVE-2018-1000810".parse::<advisory::Id>().unwrap();
    let example_advisory = db.find(&example_advisory_id).unwrap();
    let example_package = package::Name::from("std");

    assert_eq!(example_advisory.metadata.id, example_advisory_id);
    assert_eq!(example_advisory.metadata.package, example_package);
    assert_eq!(
        example_advisory.versions.patched[0],
        VersionReq::parse(">= 1.29.1").unwrap()
    );
    assert_eq!(example_advisory.metadata.date.as_str(), "2018-09-21");
    assert_eq!(
        example_advisory.metadata.url.as_ref().unwrap(),
        "https://groups.google.com/forum/#!topic/rustlang-security-announcements/CmSuTm-SaU0"
    );
    assert_eq!(
        example_advisory.metadata.title,
        "Buffer overflow vulnenrability in str::repeat()"
    );
    assert_eq!(
        &example_advisory.metadata.description[0..30],
        "The Rust team was recently not"
    );
    assert_eq!(
        example_advisory.metadata.collection.unwrap(),
        package::Collection::Rust
    );
}

/// Regression test for cloning into an existing directory
#[test]
fn clone_into_existing_directory() {
    // Make an empty temporary directory
    let tmp = tempdir().unwrap();

    // Attempt to fetch into it
    Repository::fetch(DEFAULT_REPO_URL, tmp.path(), true).unwrap();
}
