---
name: task-session-handoff
description: "Write a handoff file in the git directory of this project, in which each fact carries the command that proves it, thus a new session checks the state and does not trust it. Use when the user asks for a handoff, a summary for a different session or a different agent, when the context of the session is almost full, or when the user stops work that a later session continues."
disable-model-invocation: true
---

# Session Handoff

Write the state of the work in one file that a new session checks before it continues. Do each section of this file in its order.

## 1. Collect the state

Run `git status --short`, `git log --oneline -10` and `git diff --stat`.

List the goal of the work, each decision that the user made, each step that is done, and each step that remains.

## 2. Give each fact its proof

Give each step that is done the command that proves it, such as the name of a test that passes, a `curl` request or a `grep` that finds the new line.

Give each step that remains the command that fails today and passes when the step is done.

Write each decision of the user in the words of the user.

## 3. Write the file

Run `git rev-parse --git-dir`. Write the file to `<git-dir>/hod-handoff-<name>.md`, thus git does not track it and it stays with this clone. Give `<name>` two or three words with a hyphen between them.

Write the goal in one sentence at the top. Write each step as an item of a task list in GitHub flavored Markdown: `- [x]` for a step that is done and `- [ ]` for a step that remains, each with its command of section 2.

Write the name of a variable of the environment in place of its value.

## 4. Name the next step

Name the `hod` command that the next session runs first, such as `hod task-code-feature`.

Tell the next session in the file to run each command of the task list before it changes code, and to stop and tell the user when one result differs from the file.

## 5. Copy the prompt

Write the prompt of the next agent from this template, and replace `<path>` with the absolute path of the file:

```text
Read <path>. Run each command of its task list before you change code. When one result differs from the file, stop and tell me. Then do the first step that remains.
```

Copy the exact prompt to the clipboard with the first command that the system holds: `pbcopy` on macOS, `clip.exe` on Windows or WSL, `wl-copy`, `xclip -selection clipboard` or `xsel --clipboard --input` on Linux. Pass the prompt to the command on its standard input, with no other text.

## 6. Report

Give the path of the file.

Give the exact prompt of section 5 in a code block with the language `text`. The code block holds the same text as the clipboard, character for character.

Say that the clipboard holds that prompt. When no command of section 5 is available or the command fails, say that the copy failed.

Tell the user to start the next session with the command of section 4 and to paste the prompt.
