---
name: task-pulls-triage
description: "Triage the open pull requests of this repository: review each one against the rules of this project, ask for each change that it needs, and close each stale or duplicate one, with the approval of the user for each public action. Use when the user asks to triage, review, answer or clean the pull requests, to close stale pull requests, or to reply to contributors in the voice of the user."
disable-model-invocation: true
---

# Pulls Triage

Propose an action for each open pull request, then run the actions that the user approves. Merge no pull request. Do each section of this file in its order.

## 1. Read the project

Run `gh repo view --json nameWithOwner,url` and `gh api user --jq .login`. Stop and tell the user to run `gh auth login` when a command fails.

Read `.hod/PROJECT.md` and `CONTRIBUTING.md`.

Ask the user for the number of days after which a pull request that waits for its author is stale, and use 30 days when the user gives none.

## 2. Learn the voice of the user

Run `gh search prs --repo <owner>/<name> --commenter <login> --limit 20 --json url`, and read the comments and the reviews of the user in those pull requests.

Copy the length, the greeting, the tone and the sign-off of those comments in each reply that you write.

## 3. Read the pull requests

Run `gh pr list --state open --limit 100 --json number,title,body,author,isDraft,createdAt,updatedAt,mergeable,reviewDecision,statusCheckRollup,closingIssuesReferences`.

Give each pull request one class:

- **stale:** it waits for its author longer than the days of section 1.
- **duplicate:** a different open or merged pull request makes the same change.
- **blocked:** it has a conflict or a check that fails. Name the conflict or the failed job with `gh pr checks <number>`.
- **ready to review:** each other pull request that is not a draft.

## 4. Review each pull request that is ready

Read the change with `gh pr diff <number>`, and read the issue that it closes.

Call the Skill tool with "rules-code-without-slop".

Call the Skill tool with "rules-code-type-safety".

Call the Skill tool with "rules-code-tests".

Call the Skill tool with "rules-code-boundaries".

Call the Skill tool with "rules-code-security".

Give each finding one line: the path, the line, the rule and the change that it needs. Give no finding without a line of the diff.

Propose an approval when the change builds what its issue asks, its tests prove it, and no finding remains. Propose a request for changes in each other case.

## 5. Write the proposals

Write one row for each pull request: the number, the title, the class, the action and the full text of the review or of the reply.

Thank the contributor in each reply. Give each request for changes each finding of section 4. Give each closed pull request the reason, and link the duplicate when there is one.

Write no promise of a date and no promise of a merge.

## 6. Ask the user

Show the table, and ask the user with the question tool of the client: run each action, run the actions of some numbers, or run none. Ask for the numbers when the user selects some.

Run no action that the user did not approve, because each action is public.

## 7. Run the approved actions

Run the command of each approved action:

- ready to review, approval: `gh pr review <number> --approve --body "<review>"`
- ready to review, changes: `gh pr review <number> --request-changes --body "<review>"`
- blocked: `gh pr comment <number> --body "<reply>"`
- stale or duplicate: `gh pr close <number> --comment "<reply>"`

## 8. Report

Give the count of the pull requests of each class.

Name each pull request that you reviewed, commented or closed, with its URL.

Name each pull request that you approved, and tell the user that it is ready to merge.

Name each proposal that the user refused.
