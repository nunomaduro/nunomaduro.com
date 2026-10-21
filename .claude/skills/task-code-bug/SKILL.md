---
name: task-code-bug
description: "Find the cause of a bug from a test that fails, from a version without the bug and from the path of the wrong value, then repair it. Use when the user reports a bug, an error, a crash, a wrong value or a test that fails, when a behavior changed and nobody knows why, or when a task needs the cause of a fault before a change."
disable-model-invocation: true
---

# Code Bug

Find the cause of the bug before you change code. Do each section of this file in its order.

## 1. Write the bug report

Write the bug in three lines: the input, the output that the user expected, and the output that the user got. Ask the user for each line that the request does not give, and wait for the answer.

## 2. Write the test that fails

Call the Skill tool with "rules-code-tests".

Write a test that gives the input of section 1 and asserts the expected output, at the place that skill gives. Run it, and see that it fails with the output that the user got.

Write a script in `$TMPDIR` that shows the bug when no test can reach it, such as a bug in the build or in the installation. Ask the user for more facts when the test or the script does not show the bug. Change no code of the program before one of them shows it.

## 3. Find a version without the bug

Ask the user for a commit, a tag or a release without the bug when the request does not name one. Run `git worktree add "$TMPDIR/hod-bug-good" <good>`, copy the test of section 2 into that worktree, and run the test there.

When the test passes there, run `git bisect start HEAD <good>` in that worktree. Run `git bisect run` with a script in `$TMPDIR` that copies the test into the worktree and runs it. Read the diff of the first bad commit that `git bisect` gives.

Go to section 4 when the user knows no version without the bug, or when the test fails in that version too.

## 4. Follow the wrong value back

Start at the line that gives the wrong output. Find the place where that value gets its value, and see whether the value is already wrong there. Go back one place at a time, through each caller and each write, until you find the first place where the value is wrong.

See each value in the debugger of the language or in a temporary assertion. Keep a list of each file that holds a temporary assertion.

The first wrong place is the cause. When `git bisect` gave a commit, see that the diff of that commit touches the cause.

## 5. Repair the cause

Repair the first wrong place of section 4, not the line that shows the symptom. Change no line that the repair does not need.

Run the test of section 2, and see that it passes. Delete each temporary assertion of the list of section 4. Run `git diff`, and see that it holds only the test and the repair.

Run `git bisect reset` in the worktree when you started `git bisect`, then run `git worktree remove --force "$TMPDIR/hod-bug-good"`.

## 6. Find the same fault in a different place

Search the code for each place that does the same operation as the cause, such as a different caller of the same function or a copy of the same calculation. Write a test for each place that has the same fault, and repair it.

## 7. Run each check

Call the Skill tool with "rules-code-tooling".

Run each check that it names after your last change.

## 8. Report

Give the three lines of section 1.

Give the name of the test of section 2, its result before the repair and its result after the repair.

Give the commit that `git bisect` found, or say why section 3 found no commit.

Give the path and the line of the cause, and each place that section 4 went through to find it.

Name each place of section 6 with the same fault, or say that you found none.

Give the result of each check, and say that you made no commit.
