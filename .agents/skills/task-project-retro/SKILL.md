---
name: task-project-retro
description: "Turn the faults of a session into checks and rules of this project, and keep the count of the rules. Use when the user asks for a retrospective, asks what went wrong in a session, asks to learn from a mistake, or asks to stop a fault that came back, and when a session ends after a correction from the user."
disable-model-invocation: true
---

# Project Retro

Turn each fault of the session into the smallest change that stops it. Do each section of this file in its order.

## 1. List the faults

Read the session: the messages of the user, the commands, their output and the diff. Read the log file of the session when the user gives its path.

Write each fault on one line: the decision that was wrong, the message, the command or the output that showed it, and the correction that followed. A correction from the user is a fault.

Write no fault that you imagine. A fault needs a line of the session that shows it.

## 2. Choose a check or a rule for each fault

Give each fault that a program can find a check, such as a test, a rule of the static analyser or a rule of a test of the architecture. A check stops each later agent, and a sentence does not.

Give each other fault one rule in the form of section 2 of `AGENTS.md`.

Write no rule for a fault that a rule of this project or a skill already covers. Name that rule or that skill instead, and name the reason that the session did not obey it.

## 3. Keep the count of the rules

Name one rule that the project does not obey or that a check now covers, for each new rule. Propose to delete it.

## 4. Ask the user

Ask the user with the question tool of the client to accept each check, each rule and each delete of section 2 and section 3. Give each fault with its proposal.

## 5. Write what the user accepted

Write each check that the user accepted, and run it. See that it fails on the fault and passes on the code of today.

Call the Skill tool with "rules-code-tooling" for each check.

Write each rule and delete each rule that the user accepted, in the form of section 2 of `AGENTS.md`. Run `hod update --project` after the write.

## 6. Report

Give the list of the faults of section 1.

Name each check that you wrote, and give its path and its result.

Name each rule that you wrote and each rule that you deleted.

Name each proposal that the user refused.

Say that you made no commit.
