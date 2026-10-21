---
name: task-issues-triage
description: "Triage the open issues of this repository: close each duplicate and each outdated issue, answer each question, and label each issue that stays open, with the approval of the user for each public action. Use when the user asks to triage, clean, maintain or answer the issues, to close duplicates or stale issues, or to reply to the issues in the voice of the user."
disable-model-invocation: true
---

# Issues Triage

Propose an action for each open issue, then run the actions that the user approves. Do each section of this file in its order.

## 1. Read the project

Run `gh repo view --json nameWithOwner,description,url` and `gh api user --jq .login`. Stop and tell the user to run `gh auth login` when a command fails.

Read `.hod/PROJECT.md`, `README.md`, `CONTRIBUTING.md` and `SECURITY.md`. Find the versions that the project supports, and the labels with `gh label list`.

Ask the user for the number of days after which an issue that waits for its author is outdated, and use 30 days when the user gives none.

## 2. Learn the voice of the user

Run `gh search issues --repo <owner>/<name> --commenter <login> --limit 20 --json url`, and read the comments of the user in those issues.

Copy the length, the greeting, the tone and the sign-off of those comments in each reply that you write.

## 3. Read the issues

Run `gh issue list --state open --limit 200 --json number,title,body,labels,author,createdAt,updatedAt,comments`.

Give each issue one class:

- **duplicate:** a different issue reports the same fault or asks for the same change. Search with `gh issue list --state all --search "<words>"`, and keep the older issue or the issue with more facts.
- **fixed:** a merged pull request or a commit repairs the issue. Search with `gh pr list --state merged --search "<number>"` and `git log --grep "#<number>"`.
- **outdated:** the issue waits for its author longer than the days of section 1, or it reports a version that the project no longer supports.
- **question:** the docs or the code answer the issue.
- **open:** each other issue. Give it a label, and give a short reply when the issue has no reply of a maintainer.

## 4. Write the proposals

Write one row for each issue that needs an action: the number, the title, the class, the action and the full text of the reply.

Give each closed issue a reply that says why: link the original issue, the pull request that repaired it, or the page of the docs. Thank the author.

Write no promise of a date and no promise of a feature. Write no reply for an issue that a maintainer answered after its last comment.

## 5. Ask the user

Show the table, and ask the user with the question tool of the client: run each action, run the actions of some numbers, or run none. Ask for the numbers when the user selects some.

Run no action that the user did not approve, because each action is public.

## 6. Run the approved actions

Run the command of each approved action:

- duplicate: `gh issue close <number> --duplicate-of <original> --comment "<reply>"`
- fixed: `gh issue close <number> --reason completed --comment "<reply>"`
- outdated: `gh issue close <number> --reason "not planned" --comment "<reply>"`
- question: `gh issue comment <number> --body "<reply>"`, then close it when the user approved the close
- open: `gh issue edit <number> --add-label "<label>"`, and `gh issue comment <number> --body "<reply>"` when the row holds a reply

## 7. Report

Give the count of the issues of each class.

Name each issue that you closed, commented or labeled, with its URL.

Name each proposal that the user refused.

Name each bug that you could not class, and tell the user to run `hod task-code-bug` for it.
