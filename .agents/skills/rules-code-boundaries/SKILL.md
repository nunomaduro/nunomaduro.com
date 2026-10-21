---
name: rules-code-boundaries
description: "The rules for each value that enters the program and for the access to each record. Use when you write or change an entry point such as a controller, a handler of a request, a component that a page calls, a command or a job, when a method receives an argument from a request, a route, a form, a queue or the command line, when you convert a value to an enum, a date or a number, when you write a validation, when you write a record from a request, when you load a record by an id, when you authorize an action, and when you read a value of the environment."
---

# Code Boundaries

Obey each rule of this file each time that you write an entry point and each time that you change an entry point.

## 1. Treat each value from outside as input

Treat each value that the program does not write itself as input: each argument of an action that a page calls, each property that a page sends back, each parameter of a route, the query string, the body of a request, the payload of a job, the body of a webhook, each argument of the command line and each value of the environment.

Treat a value that the page sends back as input, also when the page wrote it, such as the id in a button or the value of a select.

## 2. Validate each input before you convert it

Validate each input with the validator of the project before you convert it to a type of this project. Check that a status is one of the cases of its enum, then convert it to that enum.

Convert no raw value to an enum, a date, a number or a structure before the validator reads it.

Give each input that holds a list or a size a maximum, such as the count of the items and the size of a page.

## 3. Refuse a wrong input with an answer of the client

Answer a wrong input with 422 and the message of the validation, a record that the user cannot reach with 404 or 403, and a guest with 401 or a redirect to the login page.

Let no wrong input reach an exception that gives 500. A 500 is a fault of the program, and a wrong input is a fault of the user.

## 4. Write only the validated fields

Write to a record only the fields that the validator gave back.

Copy no whole request into a record, and turn off no protection of the framework against the write of each field of a request.

## 5. Authorize each action in the method that runs it

Authorize each action in the method that runs it. An authorization when the page loads does not cover a later call to an action of that page.

Write each policy to refuse by default: give back `true` only for each actor that the request names.

## 6. Load each record through its parent

Load each record by its id through its parent, such as the tasks of the project of the request, thus a record of a different parent gives 404.

Let no page change a property that holds an id or a record. Mark that property read-only with the tool of the framework, or read the id again from the server.

## 7. Read the environment in one place

Read each value of the environment in the configuration module of the project, and read the configuration in each other place, because a cache of the configuration can hide the environment.

## 8. Report

Name each input that your change reads, and give the rule of the validation that reads it before its conversion.

Name each action that your change adds, and give the line that authorizes it.

Name each record that your change loads by an id, and give the parent that it loads through.

Name each property of a page that holds an id, and say how the page cannot change it.

Write this report in the last message of your answer. Write no sentence for a rule of this file that your work did not touch.
