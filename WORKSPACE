load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

# https://bazelbuild.github.io/rules_rust/
http_archive(
    name = "rules_rust",
    sha256 = "2466e5b2514772e84f9009010797b9cd4b51c1e6445bbd5b5e24848d90e6fb2e",
    urls = ["https://github.com/bazelbuild/rules_rust/releases/download/0.18.0/rules_rust-v0.18.0.tar.gz"],
)

load("@rules_rust//rust:repositories.bzl", "rules_rust_dependencies", "rust_register_toolchains")

# To install rule_rust's dependencies.
rules_rust_dependencies()

# To register toolchains to support different platforms.
rust_register_toolchains(
    edition = "2021",
    versions = [
        "1.67.1",
    ],
)

# https://bazelbuild.github.io/rules_rust/crate_universe.html
load("@rules_rust//crate_universe:repositories.bzl", "crate_universe_dependencies")

crate_universe_dependencies(rust_version = "1.67.1")

load("@rules_rust//crate_universe:defs.bzl", "crates_repository")

crates_repository(
    name = "crate_index",
    cargo_lockfile = "//:Cargo.lock",
    # The generated lock file by Bazel
    lockfile = "//:cargo-bazel-lock.json",
    # All manifest files need to be included here.
    manifests = [
        "//:Cargo.toml",
        "//protogen:Cargo.toml",
        "//routeguide-client:Cargo.toml",
        "//routeguide-consumer:Cargo.toml",
        "//routeguide-server:Cargo.toml",
    ],
)

load("@crate_index//:defs.bzl", "crate_repositories")

crate_repositories()
