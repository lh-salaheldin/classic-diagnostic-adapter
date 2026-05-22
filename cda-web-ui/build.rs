// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 The Contributors to Eclipse OpenSOVD (see CONTRIBUTORS)
//
// See the NOTICE file(s) distributed with this work for additional
// information regarding copyright ownership.
//
// This program and the accompanying materials are made available under the
// terms of the Apache License Version 2.0 which is available at
// https://www.apache.org/licenses/LICENSE-2.0

use std::{path::PathBuf, process::Command};

#[cfg(windows)]
const NPM: &str = "npm.cmd";
#[cfg(not(windows))]
const NPM: &str = "npm";

fn main() {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    let ui_dir = PathBuf::from(&manifest_dir).join("../cda-ui");

    // Re-run this build script when the UI source changes.
    println!("cargo:rerun-if-changed=../cda-ui/src");
    println!("cargo:rerun-if-changed=../cda-ui/index.html");
    println!("cargo:rerun-if-changed=../cda-ui/package-lock.json");
    println!("cargo:rerun-if-changed=../cda-ui/vite.config.ts");

    let npm_ci = Command::new(NPM)
        .args(["ci"])
        .current_dir(&ui_dir)
        .status()
        .expect("failed to run `npm ci` — is Node.js installed?");
    assert!(npm_ci.success(), "npm ci failed");

    let npm_build = Command::new(NPM)
        .args(["run", "build"])
        .current_dir(&ui_dir)
        .status()
        .expect("failed to run `npm run build`");
    assert!(npm_build.success(), "npm run build failed");
}
