---
name: task-code-plan
description: "Turn a request into decisions, requirements and slices before code. Use when the user asks to plan a feature, a change or a project, when a request leaves decisions open, when a request is too large for one session, or when the user asks which questions to answer before work starts."
disable-model-invocation: true
---

# Code Plan

Turn the request into a plan that a different session can build. Write no code of the program. Do each section of this file in its order.

## 1. Read the request and the project

Ask the user for the request and wait for the answer when the user gives none.

Read `.hod/PROJECT.md` and the code that the request touches. Take each fact from a file, and ask the user no question that a file answers.

## 2. Write a draft with your assumptions

List each decision that the request leaves open and that the code does not decide. Write your assumption for each decision, and write the file or the convention of the project that it comes from.

Write the draft of the requirements and of the slices of sections 4 and 5 on these assumptions, and keep it in your message.

## 3. Confirm the assumptions that change the draft

Mark each assumption that changes a requirement or a slice of the draft when it is wrong. Ask the user each marked assumption with the question tool of the client, in the smallest number of calls that the tool permits. Give your assumption as the first option, and the other options after it.

Keep each assumption that is not marked, and name it in the plan as an assumption.

Correct the draft for each answer that differs from your assumption.

## 4. Write the requirements

Write each requirement in one sentence that a user can check: a page shows a value, a command writes a file, a user cannot open a record.

Name the actors, each permission, each limit and each empty state of the request in the requirements.

## 5. Cut the work into slices

Cut the requirements into slices. Give each slice each layer that it needs, from the database to the page or the command, thus each slice works and passes its tests alone.

Order the slices, and name the slice that each slice needs first.

Name no path of a test file in the plan, because the skill "rules-code-tests" gives each path when the code is written.

## 6. Write the plan

Write the plan to `.hod/plans/<name>.md`. Give `<name>` two or three words with a hyphen between them. Give the file the decisions with the answers of the user, the assumptions that the user did not confirm, the requirements and the slices.

## 7. Report

Give the path of the plan and the count of the decisions, of the assumptions, of the requirements and of the slices.

Tell the user to run `hod task-code-feature` and to give it the path of the plan, one slice at a time.
