This repo is loosely based on https://github.com/hyperium/tonic/blob/master/examples/routeguide-tutorial.md

- When creating a new Cargo workspace, make sure to `git init`. If the workspace directory is not
a git repository, anytime you run `cargo new` to add a new package to your workspace it will
generate a nested git repository inside that package, which is not what we want (we want to version control
the entire workspace as a whole).
  - Alternatively, you can use `cargo new --vcs none`.