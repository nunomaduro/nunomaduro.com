---
name: rules-code-error-messages
description: "The rules for the text of a message that a person reads. Use when you write the message of an exception, of an error or of a panic, when you write a log line, when a message can hold a secret or personal data, when you write a line that a program prints for a user, when a message interpolates a value such as a path, a name, a number or a version, when a message names a command that the reader must run, and when a message names an option or a variable of the environment."
---

# Code Error Messages

Obey each rule of this file each time that you write a message and each time that you change a message.

A message is the text of an exception, of an error, of a panic, of a log line, and of each line that a program writes for a person to read. A string that the code alone reads is not a message.

## 1. Put each value inside square brackets

Write square brackets around each value that a message interpolates. The reader thus sees where the value starts and where it ends, and a value that is empty, that holds a space, or that ends with a full stop stays legible.

Write the brackets in the template of the message, and not around the value at the call site. The template of `cannot read [%s]`, of `cannot read [{path}]` and of `cannot read [${path}]` each carries the two brackets.

## 2. Put each command, each option and each name of a variable inside square brackets

Write square brackets around a command that a message names for the reader to run, such as `[composer install]` and `[git status]`. Write square brackets around the name of an option, such as `[--force]`, and around the name of a variable of the environment, such as `[PATH]`.

Write one pair of brackets around a command that interpolates a value, such as `[git checkout %s]`, and write no second pair around that value. Write no brackets around an option that stands inside a bracketed command, such as `[git commit --amend]`.

## 3. Write no bracket in four places

Write no bracket around a placeholder that carries markup for a terminal, such as `<fg=%s>` in PHP and the escape sequence `\x1b[%dm` in a shell. Write no bracket around a placeholder that holds a whole styled line, such as `<fg=gray>%s</>`. Write no bracket around a value that carries its own unit, such as a size and the name of its unit. Write no bracket around a value that already stands inside a delimiter of its own, such as the numbers of the header of a unified diff, `@@ -1,4 +1,5 @@`, and a segment of a template of a route, `{id}` in `/users/{id}`.

## 4. Write one mark for one value

Write no backtick, no quotation mark and no parenthesis around a value of a message. Two marks for one purpose give the reader two forms to learn, and square brackets are the one form of this rule. Write `%s` and not `%q` in Go, because `%q` writes its own quotation marks.

## 5. Keep each secret out of each message

Write no secret, no token, no password and no personal data into a message, and write the id of the record instead. Call the Skill tool with "rules-code-security" for the other places of a secret.

## 6. Report

Name each message that you wrote and each message that you changed, and give the text of each one.

Say that each value, each command, each option and each name of a variable of the environment in those messages carries square brackets. Name each place of section 3 that your work met, and say that you wrote no bracket there.

Say that no message of your change holds a secret or personal data.

Write this report in the last message of your answer. Write no sentence for a rule of this file that your work did not touch.
