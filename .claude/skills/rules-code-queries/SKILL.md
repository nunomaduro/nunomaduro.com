---
name: rules-code-queries
description: "The rules for each query that reads a database. Use when you write a list, a page or a report that reads rows, when a loop reads a relation of each row, when you count or sum rows, when a user chooses the size of a page, when you read all the rows of a table, when you filter or order rows by a column, and when a page or a test is slow."
---

# Code Queries

Obey each rule of this file each time that you write a query and each time that you change a query.

## 1. Load each relation that a list reads in the same query

Load each relation that a list reads for each row in one query with the list, not in one query for each row. A list of 50 tasks that reads the assignee of each task makes two queries, not 51.

Turn on the mode of the framework that stops a lazy load in development and in the tests, when the framework has one.

## 2. Bound each query that a user can grow

Read the rows of a table that grows in pages or in chunks. Read no whole table into memory.

Give the size of a page that a user chooses a maximum.

## 3. Count and sum in the database

Count, sum and group the rows in the query. Load no rows to count them in the code.

## 4. Give each filtered column an index

Give an index to each column that a query filters, joins or orders on a table that grows.

## 5. Prove the count of the queries

Assert the count of the queries in a test of each list, such as a test that adds rows and asserts that the count does not grow with them.

## 6. Report

Name each list that your change writes, and give the relations that it loads with the list.

Name each query that reads a table that grows, and give its bound.

Name each index that your change adds, and give the query that needs it.

Name each test that asserts the count of the queries.

Write this report in the last message of your answer. Write no sentence for a rule of this file that your work did not touch.
