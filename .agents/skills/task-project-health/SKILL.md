---
name: task-project-health
description: "Keep this project current and keep each check passing. Raise each dependency, repair each vulnerability, keep the lock file in sync, remove each unused package, check the runtime and the toolchain, and raise each action of the CI. Use when the user asks to upgrade the dependencies, to raise one package to a new version, to bump each version in a manifest, to repair a vulnerability of a dependency, to remove an unused package, to update the CI, to make the CI pass, or to check the health of this project."
disable-model-invocation: true
---

# Project Health

Bring this project to the newest healthy state, one step at a time. Repair each fault that you find. Ask the user one question in section 6, before the first change that the user cannot see in a test: the removal of a package or the raise of a major version. Ask no other question: the user reads the report of section 14 and decides there.

Do each section of this file in its order. When the user asks for one part of this work, such as one package or the CI alone, do sections 1, 2 and 13, the sections of that part, and section 14.

## 1. Read the project

Read `.hod/PROJECT.md` for the command that installs the dependencies, the command that runs the tests and the command of each other check.

Find each manifest at the top of the project. Section 3 names the commands of each manifest.

Find each file of the CI: `.github/workflows/*.yml`, `.gitlab-ci.yml`, `.circleci/config.yml` and `bitbucket-pipelines.yml`.

Run `git status --short`, and keep each change that it names. Copy each manifest and each lock file into a new directory outside the project before your first change. Return a manifest or a lock file from that copy, and not with `git checkout --`, because `git checkout --` deletes a change of the user.

Read the last run of the CI when `gh` is on the `PATH` and `git remote get-url origin` names `github.com`. Run `gh run list --branch "$(git branch --show-current)" --limit 1`, and run `gh run view <id> --log-failed` when that run failed. Keep each job that failed and its first error for section 12. Write no push, start no run and write no comment, because each of those reaches a different person.

When `.hod/PROJECT.md` says that this project reaches no network, write no request to a network in this work, and name each section that needed the network in the report.

## 2. Take the baseline

Write no test during this work. A test that you write now covers the code that you changed, thus it shows no regression.

Call the Skill tool with "rules-code-tooling", and run each check that it names, before you change a file. Decide this step from the output of those commands only. Read no test file to decide it, and judge no test by its value, because a test that the framework wrote is a test and a test that asserts a constant is a test.

When a check fails, repair the fault, then run each check again before you raise a package. Name each check and each test that failed from the output of the command, and the repair, in the report. When the fault is in the tool on this computer and not in the project, such as a binary that reads its arguments in a different way from the binary of the CI, install the version of the tool that the project names again, and name the fault in the report when it stays.

When the test command does not exist, or when the command runs zero tests, continue the work, and say in the report that you found no test and that no test showed a regression.

## 3. The commands of each manifest

| Manifest | Report | Raise one package | Update and align | Remove one package |
| --- | --- | --- | --- | --- |
| `composer.json` | `composer outdated --direct` | `composer require <package>:^<version> --with-all-dependencies` | `composer update --bump-after-update` | `composer remove <package>` |
| `package.json` | `npm outdated` | `npm install <package>@^<version>` | `npx npm-check-updates --upgrade --target minor`, then `npm update` | `npm uninstall <package>` |
| `Cargo.toml` | `cargo upgrade --dry-run --incompatible` | `cargo add <package>@<version>` | `cargo upgrade`, then `cargo update` | `cargo remove <package>` |
| `pyproject.toml` | `uv tree --outdated` | `uv add "<package>==<version>"` | `uv lock --upgrade`, then `uv-bump` | `uv remove <package>` |
| `go.mod` | `go list -m -u all` | `go get <package>@<version>` | `go get -u ./...`, then `go mod tidy` | `go mod tidy` |
| `Gemfile` | `bundle outdated` | `~> <version>` on the line of the gem in `Gemfile`, then `bundle update <package> --conservative` | `bundle update --all` | `bundle remove <package>` |
| `*.csproj` | `dotnet list package --outdated` | `dotnet add package <package> --version <version>` | `dotnet outdated --version-lock Major --upgrade` | `dotnet remove package <package>` |

| Manifest | Check the lock file | Write the lock file | Find each vulnerability |
| --- | --- | --- | --- |
| `composer.json` | `composer validate --no-check-all --no-check-publish --no-check-version` | `composer update --lock` | `composer audit` |
| `package.json` | `npm ci --ignore-scripts` | `npm install --package-lock-only` | `npm audit` |
| `Cargo.toml` | `cargo metadata --locked --format-version 1` | `cargo update --workspace` | `cargo audit` |
| `pyproject.toml` | `uv lock --check` | `uv lock` | `uv run --with pip-audit pip-audit` |
| `go.mod` | `go mod tidy -diff` | `go mod tidy` | `govulncheck ./...` |
| `Gemfile` | `bundle install --frozen` | `bundle lock` | `bundle-audit check --update` |
| `*.csproj` | `dotnet restore --locked-mode` | `dotnet restore --force-evaluate` | `dotnet list package --vulnerable` |

Install the tool of a command that the computer does not carry: `cargo install cargo-edit` gives `cargo upgrade`, `cargo install cargo-audit` gives `cargo audit`, `uv tool install uv-bump` gives `uv-bump`, `go install golang.org/x/vuln/cmd/govulncheck@latest` gives `govulncheck`, `gem install bundler-audit` gives `bundle-audit`, and `dotnet tool install --global dotnet-outdated-tool` gives `dotnet outdated`.

Read the lock file of the project and use the package manager that wrote it, such as `pnpm` for `pnpm-lock.yaml` and `poetry` for `poetry.lock`.

## 4. Bring the lock file in sync with the manifest

Run the command that checks the lock file of each manifest. When it fails, run the command that writes the lock file, then run the install command and the tests.

## 5. Find each package that no code uses

Search the code of the project for each direct dependency of each manifest: its name, its namespace, its module and its command. Search the code, the configuration, the scripts of the manifest and the files of the CI. The directory of the installed packages holds no code of the project: `vendor/`, `node_modules/`, `target/`, `.venv/` and a directory that `.hod/PROJECT.md` names for the dependencies.

Keep a package that gives a command, a plugin of the package manager, or a class that the framework loads by discovery, such as a provider that the framework registers by discovery. A search does not find those uses.

Write the list of each other package that the search does not find. Section 6 asks the user about each of them.

## 6. Sort the work

Run the report command of each manifest. Write one list of each dependency that has a newer version.

Split the list in two groups. The first group holds each new minor version and each new patch version. The second group holds each new major version.

Ask the user with the question tool of the client which packages of section 5 to remove and which packages of the second group to raise. Give each package as an option, with its current version, its new version and each breaking change that its release notes name. Keep each package that the user does not accept, and name it in the report. When the client has no question tool, keep each of those packages and name it in the report.

Remove each package of section 5 that the user accepts, one at a time, then run the tests. Return the manifest and the lock file from the copy of section 1 when a test fails, and keep that package.

## 7. Raise the first group in one step

Raise each package of the first group, then run the tests.

## 8. Raise one major version at a time

Read the release notes of the package between the two versions, in the `CHANGELOG.md` of the installed package or in the repository of the package. The notes name a member in a qualified form, such as `Class::method()`, and the code of the project calls that member in a different form, such as `$object->method()`, thus search for the bare name of each class, each method, each function and each option that the notes name, and not for the qualified string of the notes. A search of each bare name over the code of the project decides this step, and the directory of the installed packages holds no code of the project.

Raise the package, apply each change that the notes name to the code of the project, then run the tests. When a test fails, read the fault, correct the code, and run the tests again.

Return the manifest, the lock file and each file of the code that you changed for that package when you cannot correct the fault, then continue with the next package. Name that package, each breaking change and each file that it touches in the report.

## 9. Repair each vulnerability

Run the command that finds each vulnerability of each manifest. Raise each package that an advisory names to the first version that repairs it, with the rules of section 7 for a minor version and of section 8 for a major version.

Keep a package when no version repairs the advisory, and name the advisory in the report.

## 10. Update each package and align the manifest

Run the update and align command of section 3 for each manifest after the last package, then run the tests. Do not skip this step when sections 7, 8 and 9 raised no package. The command raises each package, and each transitive package, to the newest version that the constraints of the manifest accept, then writes the installed version into the manifest, thus a manifest that holds `^1.0` against an installed `1.9.3` holds `^1.9.3` after this step. Run the report command without the filter of the direct packages after this step, such as `composer outdated`, and name each package that it still gives in the report.

Return the manifest and the lock file from the copy of section 1 when a test fails.

## 11. Check the runtime and the toolchain

Find each version of the runtime that the project names: the constraint of the manifest, such as `require.php`, `engines.node`, `requires-python` and the `go` line of `go.mod`, the version files, such as `.nvmrc`, `.python-version`, `.ruby-version`, `.tool-versions`, `rust-toolchain.toml` and `global.json`, and each version in the files of the CI.

Read the end of support of each version from `https://endoflife.date/api/<product>.json`, such as `php`, `nodejs`, `python`, `ruby`, `go` and `dotnet`. Name each version that the project names after its end of support, with that date.

Change no version of the runtime. The server of an application and each user of a package run that version, thus name it in the report of section 14.

## 12. Make the CI current and passing

Raise each action of each workflow of GitHub to its newest major version. Read that version with `gh api repos/<owner>/<repo>/releases/latest --jq .tag_name`. Read the release notes of the action between the two versions, and change each input that the workflow gives and that the new version removed. Keep the form of the reference: a commit stays a commit, read with `git ls-remote https://github.com/<owner>/<repo> refs/tags/<tag>`, and a tag stays a tag.

Replace each label of a runner that the provider retired with the newest label of the same system.

Repair each job that failed in the run of section 1 when the fault is in the project: a check that fails, a command of the workflow that does not exist, or a version of the runtime that the CI names and the project does not support. Run the command of that job on this computer after the repair. Name each fault outside the project, such as a secret that is absent, without a change.

Push nothing. A change to a workflow stays without a run until the user pushes it, thus say that in the report.

## 13. Run each check

Run each check of section 2 after your last change. Each check must pass.

Return the change that broke a check when you cannot correct it, and name it in the report. Keep each change that `git status --short` named in section 1.

## 14. Report

Write no commit. Leave each change in the working tree.

Give these lists, and write "none" for a list that holds nothing:

- each package that you raised, with the version before and the version after,
- each package that you kept, with the version that the project holds, the newer version and the reason,
- each package that you removed, with the search that found no use of it,
- each advisory, with the package and the version that repairs it, or with the words "no repair",
- each version of the runtime after its end of support, with that date,
- each check that failed in section 2, with the repair,
- each action of the CI that you raised, with the version before and the version after, each job of the CI that you repaired, and each fault outside the project,
- each file of the code that you changed.

Say whether the lock file was in sync, and name the command that wrote it when it was not. Give the result of the last run of the CI from section 1, or say that you could not read it.

Name the last change of this work, say that you ran each check after that change, and say that those checks passed, in one sentence of its own, whenever the first list names one package. The update and align command of section 10 is the last change of the packages. The lists, the two facts and that sentence are the report of section 14.

Give the report of section 14 in the last message of the answer. Write no other sentence about a test run in that message, because two sentences about two test runs hide which run came last.
