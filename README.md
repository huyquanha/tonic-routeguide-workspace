This repo is loosely based on https://github.com/hyperium/tonic/blob/master/examples/routeguide-tutorial.md

- When creating a new Cargo workspace, make sure to `git init`. If the workspace directory is not
a git repository, anytime you run `cargo new` to add a new package to your workspace it will
generate a nested git repository inside that package, which is not what we want (we want to version control
the entire workspace as a whole).
  - Alternatively, you can use `cargo new --vcs none`.

## Bazel integration
- Consider using Bazelisk to manage different Bazel versions
- Files generated by a rule always belong to the same package as the rule itself; it's *NOT* possible
to generate files into another package.
  - It's not uncommon for a rule's input to come from other packages though.

### External Dependencies
- Bzlmod can be enabled with `--enable_bzlmod` and needs a new file `MODULE.bazel`
- `Repository`: a directory with a `WORKSPACE/WORKSPACE.bazel` file
- `Main repository`: the repository in which the current Bazel command is being run
- `Workspace`: normally used to refer to the main repository, but is actually comprise of the main
repository and all the repository mappings (explained below)
- `Canonical repository name`:
  - Within the context of a workspace, each repository has a single canonical name. A target inside
  a repo whose canonical name is `abc` can be addressed by the label `@@abc//pac/kage:target` (double `@`)
  - The main repository's canonical name is always the empty string, that's why to refer to a target
  defined inside the main repository, you only need `//pac/kage:target`

- `Apparent repository name`:
  - The name a repository is addressable by in the context of certain other repo i.e the "nickname" of a repo
  - E.g repo with canonical name `michael` might have the apparent name `mike` in `alice` repo, but
  `mickey` in `bob` repo.
    - In the context of `alice`, a target inside `michael` can be addressed by the label `@mike/pac/kage:target` (single `@`)
    - Similarly, in the context of `bob`, the same target can be addressed by the label `@mickey/pac/kage:target`
  - In another way, this can be thought of as a *repository mapping*: Each repo (bob, alice) maintains a mapping
  from "apparent name" to "canonical name".

- `Repository rule`
  - A schema for repository definition that tells Bazel how to materialise a repo.
  - E.g `http_archive` to download an archive from URL, and `local_repository` to symlinks another local Bazel repo
  - The repos defined in a workspace are not immediately available on local disk. They need to be fetched.
    - Normally, Bazel only fetch a repo when it needs something from the repo, and the repo hasn't already been fetched.
    - If the repo's already fetched, Bazel only refetches it if its definition has changed.
  - External repositories can be found in `external` dir in the output base under its canonical name
```
ls $(bazel info output_base)/external/canonical_name 
```

#### Repository Rules (https://bazel.build/extending/repo)
- Can only be used in `WORKSPACE`
- Enable non-hermetic operation at Bazel's loading phase.
- Each rule creates its own workspace, with its own `BUILD` file and artifacts. They can e used
to depend on third-party libraries (like Maven packages) but also to generate `BUILD` files
specific to the host Bazel is running on.
- To be defined in a `.bzl` file by using `repository_rule` function and store in a global variable.
- A custom repository rule can be used just like a native one:
  - `name` attribute
  - every target in its build files can be referred as `@<name>//package:target`
  - The rule is loaded when you explicitly build it, or if it's a dependency of the build. In this case,
  Bazel will execute its `implementation` function, which describes how to create the repository, its content
  and `BUILD` files.
- In `repository_rule` you must list a map of attribute names -> definitions that the rule expect
  - The attribute value can then be accessed with `repository_ctx.attr.<attribute_name>`.
  - `name` and `repo_mapping` are implicitly added attributes
  - If attribute starts with `_` it is private and can't be set by users.
- The implementation function can return either `None` to signify the rule is reproducible given the specified
parameters, or a dict with a set of parameters for that rule that would turn that rule into a reproducible
one generating the same repository. E.g a rule tracking a git repo would return a specific commit identifier
instead of a floating branch that was originally specified, so that if the rule is run again, it will point
specifically to that commit and retrieves the exact same git repo.

- If the repository is declared as `local = True`, any changes in a dependency in the dependency graph (including
the WORKSPACE file itself) will cause an execution of the implementation function.
- The implementation function can be _restarted_ if a dependency it requests is _missing_
- For non-local repositories, only one of the following changes cause a restart:
  - `.bzl` files needed to define the repository rule changes
  - Declaration of the repository rule in the `WORKSPACE` file changes
  - Any environment variable changes that are declared in `environ` attribute of `repository_rule` function.
  - Content of any file used and referred to by a label

- Sometimes, exernal repositories can become outdated without any change to its defintion or dependencies. You can
ask bazel to refetch all external repositories unconditionally by calling `bazel sync`
  - Some rules inspect the local machine and might become outdated if local machine was upgraded. Here you can ask
  bazel to only refetch those external repositories where `repository_rule` definition has the `configure` attribute
  set. Use `bazel sync --configure` for this.

#### Bzlmod
- Does not work directly with repo definitions, but intead builds a dependency graph from modules,
run `extensions` on top of the graph and define the repos accordingly.
- `Bazel module`: a project that can have multiple versions, each of which publishes metadata about
other modules it depends on.
  - Each module must have `MODULE.bazel` at its repo root, next to `WORKSPACE`. This is the module's manifest,
  declaring its name, version, dependencies ... e.g
```
module(name = "my-module", version = "1.0")

bazel_dep(name = "rules_cc", version = "0.0.1")
bazel_dep(name = "protobuf", version = "3.19.0")
```
  - A module must only list its direct dependencies, which Bzlmod will look up in a Bazel registry - by default
  the BCR (Bazel Central Registry) https://github.com/bazelbuild/bazel-central-registry
  - Repeating this process allows Bazel to discvoer the entire transitive dependency graph before performing
  version resolution
  - Modules can also specify customised pieces of data called `tags`, which are consumed by Module extensions
  after module resolution to define additional repos.

##### Module Extension deep dives
- Similar capabilities to repo rules: they can do file I/O and send network requests.
- Among other thing, they are mostly used by Bazel to interact with other package management systems (Maven, Cargo)
while still respecting the dependency graph built out of Bazel modules.
- This is a good example to understand module extensions: https://github.com/bazelbuild/examples/tree/main/bzlmod/03-introduce_dependencies_with_module_extension
  - Basically, they are similar to repository rules that when called upon might load extra things (e.g other repositories). In the above example, inside `lib_a/deps.bzl` we declare a module extension
```bazel
data_deps_ext = module_extension(
    implementation = lambda ctx: emojis(),
)
```
  - In the root `MODULE.bazel`, we can then load this extension with `use_extension` and then select the repository we want with `use_repo`. In this case there's only one and it's named `emojis`:
```
data_deps_ext_from_a = use_extension("@lib_a//:deps.bzl", "data_deps_ext")
# The emojis repo will be accessible with the "com_foo_bar_emojis" repo name instead.
use_repo(data_deps_ext_from_a, com_foo_bar_emojis="emojis")
```
  - After this `com_foo_bar_emojis` repository is available for use and we can include the targets defined inside
  it into our BUILD file, in this case it's just `file`
```
# Count how many emojis are listed in the file
genrule(
    name = "emoji_count",
    cmd = "wc -l < $< > $@",
    outs = ["emoji_number"],
    srcs = ["@com_foo_bar_emojis//file"],
)
```

### Targets
- Can either be a file target (source file or generated file) or a rule target.
- Rule target specifies relationship between a set of input files and a set of output files. Input files
can be either source files or generated files, and can also be other rule targets.

### Package Groups
```
package_group(name, packages, includes)
```
Package group defines a set of packages and associate a label to the set, which can be referenced
in `visibility` attributes. Package group is primarily used for visibility control:
  - A publicly visible target can be referenced from every package in the source tree
  - A privately visible target can only be referenced within its own package (not subpackages)
  - In between these extremes, a target may allow access to its own package plus any of the packages
  described by one or more package groups.
  - A package is considered to be in the group if it matches `packages` attribute, or is already contained
  in one of the other package groups mentioned in the `include` attributes.