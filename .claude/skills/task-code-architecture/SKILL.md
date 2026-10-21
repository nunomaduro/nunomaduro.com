---
name: task-code-architecture
description: "Find the files that the commits of this project change together, find the decision that they share, and move that decision into one module that the user picks. Use when the user asks to improve the architecture, to find technical debt, to plan a refactoring, or to find why one change touches many files."
disable-model-invocation: true
---

# Code Architecture

Find each decision that the code spreads over many files, and give it one module. Change no code before the user picks one. Do each section of this file in its order.

## 1. Find the files that change together

Run `git log --since="1 year ago" --name-only --pretty=format:@ --no-merges`. Each `@` starts the files of one commit.

Count, for each pair of files, the commits that change the two files. Keep each pair that five commits or more change together, and that sits in two different directories.

Read `.hod/PROJECT.md`.

## 2. Find the shared decision

Read the commits of each pair with `git show --stat <commit>` and the diff of each file of the pair.

Write the decision that forces the two files to change together, such as a format of the data, a list of states, a rule of a price or the name of a provider. Delete a pair when the commits change the two files for different reasons.

## 3. Apply the rules of the modules

Call the Skill tool with "rules-code-modules".

Propose, for each decision of section 2, the module that holds it, with its interface. Obey each rule of that skill in the proposal.

## 4. Test the proposal on a past commit

Take one commit of the pair. Write the files that the commit would change if the module of section 3 existed, and count them.

Write each proposal as one block: the decision, the module and its interface, the commit, the count of the files that it changed, and the count of the files that it would change with the module.

Order the proposals by the count of the commits of section 1 that the module makes smaller.

## 5. Ask the user

Ask the user with the question tool of the client which proposal to build. Give the four first proposals of section 4 as the options.

## 6. Build the proposal that the user picked

Build the module that the user picked. Keep each behavior: run each test before the change and after the change, and see the same result.

Call the Skill tool with "rules-code-tests" for each test that the new interface needs.

Call the Skill tool with "rules-code-tooling", and run each check that it names.

## 7. Report

Give each pair of section 1 with the count of its commits.

Give each proposal of section 4 with its commit and its two counts of files.

Name the proposal that the user picked, each file that you changed, and the result of the tests before and after the change.

Say that you made no commit.
