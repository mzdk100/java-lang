//! Tests for parsing Android SDK Java source files
//!
//! This test file will parse all .java files from Android SDK sources
//! to ensure the parser can handle real-world Java code.
//!
//! Set ANDROID_HOME environment variable to point to your Android SDK installation.

use java_lang::{Result, parse_file};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Find Android SDK source directory
fn find_android_sdk_sources() -> Option<PathBuf> {
    // Try ANDROID_HOME environment variable first
    if let Ok(android_home) = std::env::var("ANDROID_HOME") {
        let sources = PathBuf::from(&android_home)
            .join("sources")
            .join("android-35");
        if sources.exists() {
            return Some(sources);
        }
        // Try other API levels
        let sources_dir = PathBuf::from(&android_home).join("sources");
        if sources_dir.exists() {
            // Find the highest API level
            if let Ok(entries) = std::fs::read_dir(&sources_dir) {
                let mut highest_api = 0;
                let mut best_match = None;
                for entry in entries.flatten() {
                    if let Ok(meta) = entry.metadata() {
                        if meta.is_dir() {
                            if let Some(name) = entry.file_name().to_str() {
                                if let Ok(api) =
                                    name.strip_prefix("android-").unwrap_or(name).parse::<u32>()
                                {
                                    if api > highest_api {
                                        highest_api = api;
                                        best_match = Some(entry.path());
                                    }
                                }
                            }
                        }
                    }
                }
                if let Some(path) = best_match {
                    return Some(path);
                }
            }
        }
    }

    // Try common Android SDK locations
    let common_paths = [
        r"C:\Users\%USERNAME%\AppData\Local\Android\Sdk\sources\android-35",
        r"C:\Android\Sdk\sources\android-35",
        r"C:\Program Files\Android\Sdk\sources\android-35",
        r"D:\Android\Sdk\sources\android-35",
        r"~/Library/Android/sdk/sources/android-35", // macOS
        r"~/Android/Sdk/sources/android-35",         // Linux
    ];

    for path in &common_paths {
        let expanded = if path.contains("%USERNAME%") {
            path.replace("%USERNAME%", &std::env::var("USERNAME").unwrap_or_default())
        } else {
            path.to_string()
        };
        let path_buf = PathBuf::from(&expanded);
        if path_buf.exists() {
            return Some(path_buf);
        }
    }

    None
}

/// Count Java files in directory
fn count_java_files(dir: &Path) -> usize {
    WalkDir::new(dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.file_type().is_file()
                && e.path()
                    .extension()
                    .map(|ext| ext == "java")
                    .unwrap_or(false)
        })
        .count()
}

/// Parse a single Java file and return error if it fails
fn parse_java_file(path: &Path) -> Result<()> {
    let _ = parse_file::<java_lang::ast::CompilationUnit>(path)?;
    Ok(())
}

/// Test that parses all Java files in Android SDK sources
#[test]
fn test_android_sdk_sources() {
    let sdk_sources = match find_android_sdk_sources() {
        Some(path) => path,
        None => {
            println!("Skipping Android SDK test - SDK not found.");
            println!("Set ANDROID_HOME environment variable to your Android SDK installation.");
            return;
        }
    };

    println!("Found Android SDK sources at: {}", sdk_sources.display());

    let total_files = count_java_files(&sdk_sources);
    println!("Found {} Java files to parse", total_files);

    if total_files == 0 {
        panic!("No Java files found in Android SDK sources directory");
    }

    let mut success_count = 0;
    let mut fail_count = 0;
    let mut errors: Vec<(PathBuf, String)> = Vec::new();

    for entry in WalkDir::new(&sdk_sources) {
        let entry = match entry {
            Ok(e) => e,
            Err(e) => {
                eprintln!("Error reading directory entry: {}", e);
                continue;
            }
        };

        if !entry.file_type().is_file() {
            continue;
        }

        let path = entry.path();
        if path.extension().map(|ext| ext != "java").unwrap_or(true) {
            continue;
        }

        // Get relative path for cleaner output
        let rel_path = path.strip_prefix(&sdk_sources).unwrap_or(path);

        match parse_java_file(path) {
            Ok(_) => {
                success_count += 1;
                if success_count % 100 == 0 {
                    println!(
                        "Progress: {}/{} files parsed successfully",
                        success_count, total_files
                    );
                }
            }
            Err(e) => {
                fail_count += 1;
                let error_msg = format!("{}", e);
                eprintln!("FAILED: {} - {}", rel_path.display(), error_msg);

                // Try to read the file and show context around the error
                if let Ok(content) = std::fs::read_to_string(path) {
                    // Extract byte offset from error message
                    if let Some(offset_str) = error_msg.split("byte offset ").nth(1) {
                        if let Some(offset_end) = offset_str.find(' ') {
                            if let Ok(offset) = offset_str[..offset_end].parse::<usize>() {
                                let start = if offset > 200 { offset - 200 } else { 0 };
                                let end = if offset + 200 < content.len() {
                                    offset + 200
                                } else {
                                    content.len()
                                };
                                eprintln!("  Context around error:");
                                eprintln!("  {}", &content[start..end].replace('\n', "\\n"));
                            }
                        }
                    }
                }

                errors.push((rel_path.to_path_buf(), error_msg));

                // Stop early if too many failures
                if fail_count >= 100 {
                    eprintln!("Too many failures, stopping...");
                    break;
                }
            }
        }
    }

    // Print summary
    println!("\n=== Parsing Summary ===");
    println!("Total files: {}", total_files);
    println!("Successful: {}", success_count);
    println!("Failed: {}", fail_count);

    // Print all errors
    if !errors.is_empty() {
        println!("\n=== Failed Files ===");
        for (path, error) in &errors {
            println!("  {}: {}", path.display(), error);
        }
        panic!(
            "{} out of {} Java files failed to parse",
            fail_count, total_files
        );
    }

    assert_eq!(
        success_count, total_files,
        "All Java files should parse successfully"
    );
}

/// Test that parses Java files from a specific package (smaller test)
#[test]
fn test_android_sdk_sample() {
    let sdk_sources = match find_android_sdk_sources() {
        Some(path) => path,
        None => {
            println!("Skipping Android SDK sample test - SDK not found.");
            return;
        }
    };

    // Try to find a small package to test (e.g., java.lang or android.os)
    let test_packages = ["java/lang", "android/os", "android/util", "java/util"];

    for package in &test_packages {
        let package_path = sdk_sources.join(package);
        if package_path.exists() {
            println!("Testing package: {}", package);

            let mut success = 0;
            let mut failed = 0;

            for entry in WalkDir::new(&package_path).max_depth(1) {
                let entry = match entry {
                    Ok(e) => e,
                    Err(_) => continue,
                };

                if !entry.file_type().is_file() {
                    continue;
                }

                let path = entry.path();
                if path.extension().map(|ext| ext != "java").unwrap_or(true) {
                    continue;
                }

                match parse_java_file(path) {
                    Ok(_) => success += 1,
                    Err(e) => {
                        failed += 1;
                        let rel_path = path.strip_prefix(&sdk_sources).unwrap_or(path);
                        eprintln!("  FAILED: {} - {}", rel_path.display(), e);
                    }
                }
            }

            println!("  Parsed: {}/{} files", success, success + failed);

            if failed > 0 {
                panic!("Package {} has {} parse failures", package, failed);
            }

            // Only test first available package
            return;
        }
    }

    println!("No standard packages found to test");
}

/// Test parsing with detailed error reporting for a single file
#[test]
#[ignore = "Manual test for debugging specific files"]
fn test_parse_single_android_file() {
    // Modify this path to test a specific file
    let test_file = PathBuf::from(r"path\to\specific\File.java");

    if !test_file.exists() {
        println!("Test file not found: {}", test_file.display());
        return;
    }

    println!("Parsing: {}", test_file.display());

    match parse_java_file(&test_file) {
        Ok(_) => println!("Success!"),
        Err(e) => {
            println!("Error: {}", e);
            panic!("Parse failed");
        }
    }
}
