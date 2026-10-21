---
name: rules-code-without-slop
description: "The rules for the code that carries no evidence, which a reader names slop. Use when you write a new method or a new function, when you finish a change, when you read a request, a file, a document or a row of a database, when you check the type of a value in the middle of the program, when you write a `try` and a `catch`, an `unwrap_or` or an `if err != nil`, when you give a value a fallback, when you add an option, a parameter, an interface or a layer for a caller that does not exist today, when you name an item `data`, `info`, `manager`, `helper`, `util`, `wrapper` or `shape`, when you write a comment, and when you read the diff of your own change."
---

# Code Without Slop

Obey each rule of this file each time that you write code and each time that you finish a change.

Slop is code that carries no evidence: one line drops a fact that the program proved, and a later line proves it again. Each rule of this file keeps one fact.

## 1. Prove the shape of an input one time, at its boundary

Read each input at the place that receives it: the request, each argument and each property that a page sends back to an action, each parameter of a route, the payload of a job, the file, the document, the answer of an interface over HTTP, the row of a database, the argument of the command line and each value of the environment. Write one function that reads that input, that gives back a type of this project, and that stops with a fault when the input does not agree with that type. Call the Skill tool with "rules-code-type-safety" for the type that this function gives back.

Write no check of a type after that place. A check of `typeof`, of `instanceof`, of `is_array`, of `isinstance` or of the presence of a key, in the middle of the program, says that the type of the parameter is wrong. Correct that type.

Write no second proof of a value that its type already gives. A parameter of a type that holds a date needs no check that the text is a date.

## 2. Write no code for a caller that does not exist

Write the code that the task of today needs. An option, a parameter, a branch, an interface, an abstract class, a factory, an event and a layer each need one caller today.

Call the Skill tool with "rules-code-modules" for an interface, a layer or a type that holds a different item.

## 3. Let a fault stop the program

Write no `try` and no `catch` around code that gives a fault that this program cannot repair. A `catch` that writes a log line and continues, an `unwrap_or` that gives a default, a `_ =` that drops a `Result`, and an `err` that a branch reads and does not return each give the caller a wrong answer in the place of a fault.

Write a `catch`, a `match` on an error or a branch on `err` for two purposes: to repair the fault, and to give the fault a message of this project and pass it to the caller. Call the Skill tool with "rules-code-error-messages" for the text of that message.

A value from a user that fails its validation is not a fault of the program. Answer it with a refusal, such as 422, 403 or 404, and not with an exception that gives 500. Call the Skill tool with "rules-code-boundaries" for it.

Write no fallback value for a value that the program needs. A configuration that is absent, a file that is absent and an answer that is empty each stop the program with a message that names the item.

## 4. Give each item a name that says what it is

Write no name that gives the kind of the item in the place of its subject: `data`, `info`, `item`, `object`, `value`, `values`, `result`, `shape`, `manager`, `helper`, `util`, `utils`, `wrapper`, `base`, `common`, `misc` and `temp`.

Write no suffix that gives the type again, such as `OrderData`, `PriceValue` and `UserObject`. The type of the signature holds that fact.

Give a function the operation that it does, and give a type the thing that it is. Read the names of the files near the file that you write, and obey the convention that they hold.

## 5. Leave no line that the task does not need

Write no comment that gives the line below it again. Put the intention in the name and in the type.

Delete each line that your change replaced: the path that no caller reaches, the import of a name that the file does not use, the name that you changed, and the test of the code that you deleted. Git holds the history, thus keep no copy of the old code in the file.

Change no line that your task does not need. Write no format of a line that you did not write, no new order of the imports, no rename of a file, and no new dependency for an operation that this project holds in ten lines. A suppression in a file that you touch is not such a line. Call the Skill tool with "rules-code-tooling" for it.

Write no test of the language and no test of a library. Test the code of this project.

Read the diff of your change before you write your report, and delete each line of it that no rule of your task needs.

## 6. Report

Call the Skill tool with "rules-code-tooling" after your change, and run each check that it names.

Name the function that proves the shape of each input that your change reads, and give the path of it.

Name each item that you did not write, because no caller needs it today.

Name each fault that stops the program, and name each `catch` that you deleted.

Name each item that you renamed, and give the two names.

Name each line that you deleted, because your change replaced it.

Say that you read the diff of your change, and name each line of it that no rule of your task needs.

Say that you ran the tests of this project, and give the result of that run.

Write this report in the last message of your answer. Write no sentence for a rule of this file that your work did not touch.
