---
name: task-project-init
description: "Write the intention of this project in `.hod/PROJECT.md`. Use when the user asks to set up this project for a coding agent, when `.hod/PROJECT.md` holds the text that `hod init` wrote, or when a task needs the command that installs the dependencies, the command that runs the tests or the command that starts the program and `.hod/PROJECT.md` names none of them."
disable-model-invocation: true
---

# Project Init

Write `.hod/PROJECT.md` from the code of this project. The `AGENTS.md` file of this project tells each model to read that file first, thus the file carries the intention and the commands of the project.

## 1. Read the file

Read `.hod/PROJECT.md`. The file must name five items: the intention of the project, whether this project is a package that a different developer installs or an application, the command that installs the dependencies, the command that runs the tests, and each directory at the top of the project. Name the file that holds the public API when this project is a package.

Continue with section 2 when the file holds the text that `hod init` wrote. That text asks the reader to write the intention, and it names no command of this project.

Continue with section 4 when the file names each of the five items. Write no other change in `.hod/PROJECT.md`.

Continue with section 2 for the item that is absent when the file names one item and not each one. Keep the text that the user wrote.

## 2. Read the project

Take each fact from a file of this project. Ask the user for a fact that no file gives, and ask no question that a file answers.

Read the manifest at the top of the project, such as `composer.json`, `package.json`, `Cargo.toml`, `pyproject.toml` or `go.mod`. It gives the name of the project, the dependencies and the scripts. The lock file names the package manager: `pnpm-lock.yaml` gives `pnpm`, and `poetry.lock` gives `poetry`.

The manifest says whether this project is a package or an application: `type: library` in `composer.json`, `private: true` in `package.json` and a `[lib]` section in `Cargo.toml`. The entry point of a package names the file that holds the public API, such as `main` in `package.json` and `src/lib.rs` in a crate.

Read `README.md`, `CONTRIBUTING.md` and the file of the continuous integration, such as `.github/workflows/ci.yml`. The workflow gives the command that the project runs on each push, thus it gives the test command that the project trusts.

Run the test command one time before you write it in the file. Correct the name when the shell reports that the command is absent. Keep the command when the tests fail, because a test that fails does not make the command wrong.

Name each directory at the top of the project. Read enough of a directory to say what it holds in one line.

## 3. Write the file

Write `.hod/PROJECT.md` in this form. Write one paragraph on one line.

```markdown
# The intention of this project

Pest is a testing framework with a focus on simplicity. A PHP developer uses it to write a test for an application or for a package. It is a package, and `src/Functions.php` holds its public API.

Install the dependencies with `composer install`. Run the tests with `composer test`. Start the program with `./bin/pest`.

- `src/` — the framework: the plugins, the expectation API and the test case
- `bin/` — the executable `pest`
- `tests/` — the tests of the framework, in Pest itself
```

Give the intention one paragraph: what the project does, who uses it, and whether it is a package or an application. Name the file that holds the public API of a package. Write the exact command in `code font`. Give one line to one directory.

Write a fact that the code holds in the code. `.hod/PROJECT.md` holds two items: the intention of the project, and a reason that the code cannot hold.

Write no rule in this file. A rule needs a fault that happened, and `AGENTS.md`, section 2, gives the place of each rule.

## 4. Ask the user

Show the text of the file and ask the user to accept it. Wait for the answer. Name each fact that you took from a file, and name each fact that you guessed.

## 5. Set up the design

Continue with section 6 when the project has no page, no component, no style and no screen that a user sees.

Call the Skill tool with "impeccable", and do its command `init`. That command writes `PRODUCT.md`.

Do the command `document` of that skill when the project holds a style, such as a stylesheet, a theme or a file of tokens. That command writes `DESIGN.md`.

## 6. Report

Give the path `.hod/PROJECT.md` and say what the file holds now.

Give the path of `PRODUCT.md` and of `DESIGN.md` when section 5 wrote them.

Say that a rule of this project goes in `.ai/rules/` through the MCP tool `record-rule` when the project holds `boost.json`, and in a file in `.hod/rules/` when it does not. Say that a skill of this project goes in a directory in `.hod/skills/`.

Tell the user to run `hod` to name each task that this project carries, and `hod skills` to name each other skill.
