---
name: task-code-review
description: "Review a change, give each finding one severity in one list, and prove each blocker and each fault with a test that fails. Use when the user asks to review a change, a branch, a diff or a pull request, when the user asks for a second opinion on code that an agent wrote, or when a task ends and the change needs a check that its author did not do."
disable-model-invocation: true
---

# Code Review

Review the change with a reviewer that did not write it. Do each section of this file in its order.

## 1. Find the change

Run `gh pr diff <number>` when the user names a pull request, and `git diff $(git merge-base <ref> HEAD)` when the user names a branch or a commit. Run `git diff HEAD` and `git status --short` when the user names none, thus the review reads each change of the working tree.

Tell the user that no change exists when the diff is empty, and stop.

## 2. Find the requirements

Read the plan in `.hod/plans/` that the user names or that the branch builds. Take the requirements of that plan. When no plan exists, take each requirement from the request of the user, from the issue or from the description of the pull request.

Write each requirement in one sentence that a user can check. Ask the user with the question tool of the client for the requirements when no source gives them.

## 3. Review the change against the rules

Start one subagent for each skill below whose description names a decision of the diff. When the client has no subagent, do each review in order. Give each subagent the diff and the name of its skill, and tell it to change no file.

Call the Skill tool with "rules-code-boundaries".

Call the Skill tool with "rules-code-data-integrity".

Call the Skill tool with "rules-code-migrations".

Call the Skill tool with "rules-code-modules".

Call the Skill tool with "rules-code-queries".

Call the Skill tool with "rules-code-security".

Call the Skill tool with "rules-code-tests".

Call the Skill tool with "rules-code-type-safety".

Call the Skill tool with "rules-code-without-slop".

Call the Skill tool with "rules-code-immutability".

Call the Skill tool with "rules-code-error-messages".

Call the Skill tool with "rules-code-public-api".

Tell each subagent to give each finding on one line in this form: `<path>:<line> | <skill> §<section> | <fault> | <repair>`. Tell it to give no finding without a line of the diff.

## 4. Find each requirement in the change

For each requirement of section 2, find the code that builds it and the test that proves it. Write a finding in the form of section 3 with `requirement` in place of `<skill> §<section>` for each requirement without code or without a test.

## 5. Give each finding its severity

Put each finding of section 3 and of section 4 in one list. Give each finding one severity:

- `blocker`: a user loses data, sees data of a different user, or cannot do a step of a requirement.
- `fault`: the code breaks a rule of a skill, or a requirement has no test.
- `note`: the code obeys each rule, and a different form is easier to read.

Order the list from `blocker` to `note`. Merge two findings at one line into one finding with the higher severity.

## 6. Prove each finding

Write one test that fails for each `blocker` and each `fault`, at the place that the skill "rules-code-tests" gives. Run it, and see that it fails.

Delete a finding when its test passes on the code as it is, because the finding is wrong.

Ask the user with the question tool of the client which findings to repair. Repair each finding that the user selects, and run its test again.

## 7. Run each check

Call the Skill tool with "rules-code-tooling".

Run each check that it names after your last change.

## 8. Report

Give the list of section 5, one finding per line, with its severity first.

Name each finding that you deleted because its test passed.

Name each finding that you repaired, and give the name of its test.

Give the result of each check.

Say that you made no commit.
