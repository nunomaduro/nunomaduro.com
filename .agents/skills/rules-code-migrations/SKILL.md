---
name: rules-code-migrations
description: "The rules for each change to the schema of a database. Use when you write a migration, when you add, rename or drop a table or a column, when you add a foreign key or an index, when a value must be unique, when you change a migration that exists, when you run the migrations, and when you choose which database a command reaches."
---

# Code Migrations

Obey each rule of this file each time that you write a migration and each time that you change the schema of a database.

## 1. Change no migration that a different computer ran

Change no migration that the main branch holds, because each database that ran it keeps the old schema. Run `git log origin/HEAD --oneline -- <path of the migration>`: a line in the output says that the main branch holds it. Write a new migration instead.

## 2. Make each migration reversible

Write the reverse step of each migration, and make it restore the schema that the migration found.

Run the migration, its reverse step and the migration again against the test database before you finish.

## 3. Choose the action of each foreign key

Give each foreign key the action that the request needs when its parent row goes: delete the child, clear the column, or refuse the delete. Write no foreign key without an action.

Give each foreign key an index.

## 4. Give each unique value a unique index

Give each value that must be unique a unique index, also when a rule of the validation checks it, because two requests can pass the validation together.

## 5. Change a schema in two steps when the old code still runs

Add a column as nullable or with a default value, thus the code that runs today can still write a row.

Rename or drop a column in two releases: the first release writes the new form and reads the two forms, and the second release drops the old form.

## 6. Read the test database from the configuration of the tests

Read the name of the test database from the configuration of the tests before you run a command that drops each table, wipes the database or runs each migration again. Do not guess it.

Run only the migrations that are new against each other database.

## 7. Report

Name each migration that your change writes, and say that you ran it, its reverse step and the migration again.

Name each foreign key, and give its action and its index.

Name each unique index.

Say that you changed no migration that the main branch holds.

Write this report in the last message of your answer. Write no sentence for a rule of this file that your work did not touch.
