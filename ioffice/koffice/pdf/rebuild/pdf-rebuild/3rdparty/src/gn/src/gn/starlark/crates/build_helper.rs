// Copyright 2026 The Chromium Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

fn require_lib(out_dir: &std::path::Path, name: &str) {
    println!("cargo:rustc-link-lib=static={name}");
    let lib = if cfg!(target_os = "windows") {
        format!("{name}.lib")
    } else {
        format!("lib{name}.a")
    };
    println!("cargo:rerun-if-changed={}", out_dir.join(lib).display());
}

fn require_libs(names: &[&str]) {
    // When running with ninja, it should set NINJA_OUT_DIR.
    // If running directly with cargo, we know nothing about the output directory,
    // so we fall back to assuming the default one.
    let out_dir = if let Ok(out_dir) = std::env::var("NINJA_OUT_DIR") {
        std::path::PathBuf::from(out_dir)
    } else {
        let manifest_dir = std::path::PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
        manifest_dir.join("../../../../../out")
    };
    println!("cargo:rustc-link-search=native={}", out_dir.display());
    for name in names {
        require_lib(&out_dir, name);
    }
}
