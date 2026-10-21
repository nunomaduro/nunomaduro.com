---
name: rules-code-data-integrity
description: "The rules for the writes to a database. Use when one operation writes two rows or more, when one operation writes to two tables or more, when an operation sends a job, a mail, an event or a request over HTTP after a write, when a value must be unique, when an operation reads a value and writes a new value from it, when you change a counter, when you delete a row that a different row points to, when you remove a user from a group, when you move a value from one row to a different row, and when the code carries `transaction`, `BEGIN`, `COMMIT`, `ROLLBACK` or `FOR UPDATE`."
---

# Data Integrity

Obey each rule of this file each time that you write code that writes to a database and each time that you change that code.

## 1. Put the writes of one operation in one transaction

Write the writes of one operation in one transaction when the operation writes two rows or more. Use the transaction of the framework or of the database library of the project.

Put each write that the operation needs in the transaction: the delete of a row, the clear of each column that points to that row, and the write to a pivot table. A fault in the second write must not leave the first write in the database.

Write no transaction around one write, because the database gives one write the same guarantee.

## 2. Send each side effect after the commit

Send each job, mail, event and notification that a write causes after the commit of its transaction. Use the option of the framework or of the queue that waits for the commit. A rollback must not leave a mail that the user already received.

Make no request over HTTP inside a transaction, because the transaction holds its locks until the answer comes.

## 3. Let the database hold each unique value

Give each value that must be unique a unique index. Call the Skill tool with "rules-code-migrations" for that index.

Write a unique row with one statement that inserts the row or gives back the row that exists, or catch the error of the unique index. Do not read the row and then write it in a second query, because two requests can both read no row.

## 4. Lock each value that you read and write again

Change a counter with one statement, such as `UPDATE tasks SET count = count + 1`, not with a read and a write.

Read a row with a lock, such as `SELECT ... FOR UPDATE`, inside the transaction when the operation writes a new value from the value that it read.

## 5. Report

Name each operation that writes two rows or more, and give the path of its transaction.

Name each side effect that the operation sends after the commit.

Name each unique value, and give the migration of its index.

Name each value that the operation locks or changes in one statement.

Write this report in the last message of your answer. Write no sentence for a rule of this file that your work did not touch.
