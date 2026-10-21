---
name: rules-git-commits
description: "The rules for the message of a commit. Use when you write a commit, when you amend the message of a commit, when you squash commits into one commit, and when you write the title of a pull request that becomes the message of a squashed commit."
---

# Git Commits

Obey each rule of this file each time that you write the message of a commit and each time that you change the message of a commit.

Each rule of this file comes from [Conventional Commits 1.0.0](https://www.conventionalcommits.org/en/v1.0.0/). Read that page when a rule of this file does not answer your question.

## 1. Write the first line as `<type>(<scope>): <description>`

Write the type, an optional scope inside parentheses, a colon, one space and the description. Write the description as one short line that says what the change does, such as `fix(parser): keep an empty field of a row`.

Write a body after one empty line when the first line cannot carry the reason of the change. Write each footer after one more empty line, in the form `<token>: <value>`.

## 2. Give the type that the change carries

Give the type `feat` to a change that adds a feature for the user. Give the type `fix` to a change that corrects a bug. Give one of the types `build`, `chore`, `ci`, `docs`, `perf`, `refactor`, `style` or `test` to a different change.

Split a change that carries two types into two commits.

## 3. Take the scope from the history of the project

Run `git log --format=%s -n 50` before you write the first commit of a task. Use a scope that the history already uses for the same part of the project. Give no scope when the change touches more than one part, or when the history uses no scope.

## 4. Mark each change that breaks a user

Write `!` before the colon, such as `feat(api)!: remove the method rows`, when the change breaks the code or the use of a user. Write the footer `BREAKING CHANGE: <description>` in the same commit, and say there what the user must change.

## 5. Report

Give the first line of each commit that you wrote, and name its type and its scope.

Name each commit that carries `!` and give its footer `BREAKING CHANGE:`.

Write this report in the last message of your answer. Write no sentence for a rule of this file that your work did not touch.
