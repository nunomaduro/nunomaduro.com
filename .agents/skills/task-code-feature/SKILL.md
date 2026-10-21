---
name: task-code-feature
description: "Build a feature from a request, with a test for each requirement and each check of the project passing. Use when the user asks to build, add or implement a feature, a page, a command, a model or an endpoint, when the user gives a list of requirements, or when a task adds a behavior that a user of the program can see."
disable-model-invocation: true
---

# Code Feature

Build the feature that the user asks for, then prove each requirement with a test. Do each section of this file in its order.

## 1. Read the request

Ask the user for the feature and wait for the answer when the request names no feature.

Read the plan when the user gives a file of `.hod/plans/`, and build one slice of it.

Call the Skill tool with "rules-code-tests".

Write the list of the requirements of the request, as section 1 of that skill says. Ask the user for a requirement that the request and the code do not decide, and ask no question that a file answers.

## 2. Read the project

Read `.hod/PROJECT.md` for the intention, the command that runs the tests and each directory of this project.

Read the code near each file that your change touches, and obey the convention that it holds: the names, the layout of the directories, the form of the tests.

## 3. Take the rules

Call the Skill tool with "rules-code-without-slop".

Call the Skill tool with "rules-code-type-safety".

Call the Skill tool with "rules-code-boundaries" when the feature adds or changes an entry point.

Call the Skill tool with "rules-code-data-integrity" when the feature writes to a database.

Call the Skill tool with "rules-code-migrations" when the feature changes the schema of a database.

Call the Skill tool with "rules-code-queries" when the feature reads a list of rows.

Call the Skill tool with "rules-code-security" when the feature runs a query with a value, runs a process, prints a value from a user, stores a file, reads a secret or adds an action that a guest can call.

Read the description of each other skill of this project, and call the Skill tool with each skill whose description names a framework, a library or a tool that the feature uses.

## 4. Build the feature

Write the code of each requirement of section 1. Write the code that the requirements need, and no code for a requirement that the list does not hold.

Write the tests of each requirement as the skill "rules-code-tests" says.

## 5. Run each check

Call the Skill tool with "rules-code-tooling".

Run each check that it names. Repair each fault that your change caused, and run the check again.

## 6. Read the diff

Run `git status --short` and `git diff`, and read each line of your change.

Put a mark on each requirement of section 1 that the diff builds and that a test proves. Return to section 4 for each requirement without a mark.

Repair each fault that you find in your change, such as an input that gives a fault of the server in the place of a refusal, and write the test that proves the repair. Leave no fault of your change open in the report.

## 7. Report

Give the list of the requirements, and give the path of the code and the name of the test of each requirement.

Give each report that the skills of section 1, section 3 and section 5 ask for.

Name each file that you changed and each command that you ran.

Say that you made no commit.

Tell the user to run `hod task-code-review` for a second review of the change.
