---
name: rules-code-security
description: "The rules for each place where a value reaches a parser, a process, a page, a file, a different server or a log. Use when you write a query with a value in it, when you run a process, when you print a value in a page, when you redirect to a URL, when you send a request to a URL that a user gave, when you store an uploaded file, when you read or pass a secret, a token or a password, when you write a log line, and when you write an action that a guest can call."
---

# Code Security

Obey each rule of this file each time that you write code that sends a value to a parser, a process, a page, a file, a different server or a log, and each time that you change that code.

## 1. Pass each value to a query as a binding

Pass each value to a query as a binding, such as `WHERE email = ?` with the value in the list of the bindings. Write no value into the text of a query.

Take the name of a column or of a direction of an order from a fixed list in the code, because a binding cannot hold a name.

## 2. Pass each argument to a process in a list

Run each process with the program and each argument as items of one list. Write no command as one string with a value in it, because the shell reads that string again.

## 3. Escape each value that a page prints

Print each value in a page with the escaping form of the template. Write the raw form only for markup that the code wrote itself, and never for a value from a user.

## 4. Send a user to a known place only

Redirect only to a route of this project or to a URL that the code checks against a fixed list of hosts.

Send a request to a URL that a user gave only after you check its host against a fixed list. Refuse each address of the local network and of the computer itself.

## 5. Store each upload under a name that the program writes

Check the type and the size of each uploaded file before you store it.

Store each upload under a name that the program generates, in a place that the web server does not serve. Use no name and no path from the client.

## 6. Keep each secret out of each message

Read each secret from the configuration. Write no secret in the code, in a test or in a fixture that the repository holds.

Write no secret, no token, no password and no personal data, such as an email address, into a log line, an exception or an answer to the client. Write the id of the record instead.

Hide each column that holds a secret or a token from each serialization of its record.

## 7. Limit the rate of each action that a guest can call

Limit the count of the calls in one period for each login, each reset of a password, each invitation and each write that a guest can make.

## 8. Report

Name each query that your change writes, and say that each value is a binding.

Name each process that your change runs, and say that its arguments are a list.

Name each value from a user that a page prints, and say that the template escapes it.

Name each redirect, each request to a URL from a user and each upload, and give the check that guards it.

Name each secret that your change reads, and say that no message holds it.

Name each action that a guest can call, and give its limit of the rate.

Write this report in the last message of your answer. Write no sentence for a rule of this file that your work did not touch.
