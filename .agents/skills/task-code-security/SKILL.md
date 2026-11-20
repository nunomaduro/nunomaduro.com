---
name: task-code-security
description: "Find each fault of this project that an attacker can use, prove each one with a test that does the attack and fails on the code as it is, and report no fault without that proof. Use when the user asks for a security audit, a security review or a penetration test of the code, when the user asks whether this project or a change is secure, when a change touches the authentication, the authorization, a payment, an upload, an outgoing request or a secret, or when a task ends and nobody checked what an attacker can do with it."
disable-model-invocation: true
---

# Code Security

Find what an attacker can do with this project, and prove it. A finding without a proof is a guess, thus report no guess as a finding. An empty list of findings is a correct result: write no finding to fill the list. Do each section of this file in its order.

Send each request of this work to the application of this project on this computer. Send no request to a server outside this computer.

Do not use a secret that you find. Write no secret into the report, into a test or into a file: write the path, the commit and the name of the variable instead.

## 1. Find the scope

Run `gh pr diff <number>` when the user names a pull request, and `git diff $(git merge-base <ref> HEAD)` when the user names a branch or a commit. Read the whole project when the user names none.

For a change, take each entry point that the change touches, and follow each value of the change to its sink across the whole project, because a hole opens where a changed value reaches old code.

Read `PROJECT.md` for the command that runs the tests and the command that starts the application for a test.

Call the Skill tool with "rules-code-security".

Call the Skill tool with "rules-code-boundaries".

Keep the number of each section of those two skills: each finding names the rule that the code breaks.

## 2. Write the map of the attack surface

Write `$TMPDIR/hod-security/map.md` with three tables. Each later section reads this file, and each subagent receives its path.

Table 1 holds each entry point: each route, each command, each job, each listener of a queue, each webhook, each channel of a websocket, each scheduled task and each action of a page component. Give each one its path, its method, its guard, such as none, a session, a token, a signature or a role, and the path and the line of its handler. Find each route in the files that register routes, in the attributes of the handlers, and in the output of the command of this project that lists the routes, when one exists.

Table 2 holds each sink: each query whose text the program builds at run time, each process that the program starts, each raw output in a template, each path that the program opens, writes, serves or includes, each outgoing request, each redirect, each deserialization of a value, each template that the program compiles from a value, each header of a response, each log line and each mail. Give each one its path and its line.

Table 3 holds each actor: a guest, a user with an account that each person can create, a user of a different organization or tenant, an administrator of one organization, the sender of a webhook and a provider of a login. Give each one the factory, the seeder or the helper with which the tests of this project create it. The operator of the server and a person who holds a right are not actors for that right. A different actor can still attack them, such as a user who stores markup that the page of an operator prints.

## 3. Write a candidate, not a finding

A candidate is one line in this form: `<actor> | <entry point> | <input> | <gain> | <path of the value>`. The gain is one of these: reads a record of a different actor, writes or deletes a record of a different actor, takes a right that the actor does not have, runs code or a command on the server, reads or writes a file of the server, makes the server send a request to an address of the actor, takes the account of a different actor, reads a secret, or exceeds a limit of this project. The path of the value is each `<path>:<line>` from the entry point to the sink or to the absent check.

A pattern in the code, such as a query with text built at run time, is a place to look, and not a candidate. A rule of "rules-code-security" that the code breaks without a gain is not a candidate: tell the user to run `hod task-code-review` for it. A version of a dependency with an advisory is not a candidate: tell the user to run `hod task-project-health` for it.

Start one subagent for each of the sections 4 to 10 when the client has subagents, and do each section in order when it has none. Give each subagent the path of the map and the text of its section. Tell it to change no file and to give each candidate in the form of this section.

When a value passes through a function of a dependency, read that function in the directory of the installed packages, and name the line that binds, escapes, checks or passes the value. Decide from that line, and not from the name of the function.

## 4. Hunt in the access to each record

For each entry point of table 1 that loads a record by an id from the route, the query string, the body or a header, find the line that ties the record to the actor: a condition on the owner, the tenant or the team of the record, or an authorization check that reads the record. Write a candidate when no line ties it. Read each id of the request, and not the id of the route alone: a body that names a parent, a related record, a file or a recipient carries an id too.

For each entry point with a guard of a role, find the line that reads the role. Write a candidate when a handler reads a role, a tenant, an owner or a price from the body of the request.

For each write, list each field that the request can set. Write a candidate when the client can set a field that the server owns, such as an owner, a role, a price, a total, a balance, a verified mark or an id of a different actor.

For each list, count, search and export, find the condition of the actor in the query. Write a candidate when one of them gives a row of a different actor.

For each operation that reads a balance, a stock or a count and writes a new value from it, find the lock or the atomic write. Write a candidate when two requests at one time can spend one value two times.

## 5. Hunt in each value that reaches a sink

For each sink of table 2, follow the value back to its entry point, one assignment at a time, through each caller. Stop with no candidate when you reach a constant, a value that the server generates, or a value that the code checks against a fixed list. Write a candidate when the value reaches the sink with the text that an actor gave.

A value that the program stored in one request and reads in a later request is a value from an actor. Follow it through the column, the file or the queue that holds it.

Read these sinks with care: the text of a query, the name of a column or of a table in an order or a filter, the arguments of a process, the raw output of a template, a path, the address of an outgoing request, the target of a redirect, a deserialization, a template compiled from a value, and a header of a response.

## 6. Hunt in the flows of the account

Read the login, the registration, the reset of a password, the verification of an email, the invitation, the change of an email or of a password, the code of a second factor, the session, the remember token, each API token and each callback of a provider of a login.

For each token, find its source of randomness, its expiry, its number of uses and the actor that it names. Write a candidate when a token comes from a time, a sequence or a hash of a known value, when it does not expire, when it works two times, or when the handler takes the actor from the request and not from the token.

For the login, the reset, the verification and the code of a second factor, count the calls that the guard accepts in one period. Write a candidate when an actor can try each value of a code or each password of a list.

Write a candidate when a session keeps its id across a login, when a session works after a logout, or when the other sessions of an actor work after a change of its password.

Write a candidate when the host of a link in a mail comes from the `Host` header of the request and not from the configuration.

For each webhook, find the line that checks the signature with a secret from the configuration. Write a candidate when the handler reads the body before that line, or when no line checks it.

For each write with a guard of a session, find the check of the request token or the `SameSite` attribute of the session cookie in the production configuration. Write a candidate when a page of a different origin can send that write with the session.

## 7. Hunt in each secret and each personal datum

Run `gitleaks git --redact --no-banner .` for the history of the repository. `go install github.com/gitleaks/gitleaks/v8@latest` gives `gitleaks`. Write a candidate for each result that names a secret of a service, with the path and the commit. A secret that a later commit removed is a candidate too, because the history keeps it.

For each record with a column that holds a password, a token, a secret of a second factor or a remember token, find the list of the hidden columns, and find each entry point that gives that record to a client. Write a candidate when one entry point gives the column.

For each log line and each report to an external service of an error, find what it writes. Write a candidate when it writes a request body, a header, a password or a token.

Read the production configuration and the example of the environment file. Write a candidate when the production configuration turns the debug mode on, or when a secret has a default value in the code.

## 8. Hunt in each file

For each upload, find the check of the type from the content of the file, the check of the size, the name under which the program stores it and the place. Write a candidate when the program keeps the name of the client, when it stores the file in a place that the web server serves, or when it serves an uploaded page or an uploaded image with markup from the origin of the application.

For each read, write, download or include with a path or a name from a request, write a candidate when the path can leave the directory that the code intends, such as with `..` or an absolute path.

For each archive that the program extracts, write a candidate when an entry with `..` or a link can write outside the target directory.

## 9. Hunt in each outgoing request and each redirect

For each outgoing request whose address, host or path comes from an actor, find the check of the host against a fixed list and the refusal of each address of the local network, of the computer itself and of the metadata service of the cloud provider. Write a candidate when one of them is absent.

For each redirect whose target comes from an actor, write a candidate when the target is not a route of this project and no check reads its host against a fixed list. Test the forms `//host`, `\\host`, `https:host` and a host with a user name before `@`.

## 10. Hunt in the configuration of the production

Decide each candidate of this section from the files of the repository: the production configuration, the example of the environment file, the deployment scripts, the container file and the files of the CI. Read no running server for it.

Write a candidate when the origin policy of the production answers each origin or reflects the origin of the request together with credentials. Write a candidate when a route of a development tool, such as a profiler, a preview of mails or a playground of an API, is registered without a guard in the production. Write a candidate when a seeder or a script of the deployment writes an account with a password that the repository holds. Write a candidate when the session cookie of the production lacks `Secure` or `HttpOnly`. Write a candidate when a write that a guest can call and that costs the server a mail, a message or a row has no limit of the rate.

## 11. Disprove each candidate

Start one subagent for each candidate when the client has subagents, and do the work in order when it has none. Give it the candidate, the path of the map and no opinion of yours. Tell it to find the line of this project or of a dependency that stops the attack, such as a check, a binding, an escape, a guard, a scope or a limit, to give the path and the line, or to say that none exists, and to change no file.

Read the line that the subagent names. Delete the candidate when that line stops the attack, and keep the candidate and its line for the report. Keep the candidate when the subagent names no line, or when the line does not stop the input of the candidate, such as a check of the extension against an attack through the content.

Delete a candidate whose actor holds the gain already. Delete a candidate whose only victim is the actor. Merge two candidates with one entry point and one gain into one candidate.

## 12. Prove each candidate

Call the Skill tool with "rules-code-tests".

Write one test for each candidate at the place that skill gives. The test creates the actor and the victim with table 3, sends the input of the candidate to the entry point with the client of the test framework, and asserts that the actor does not get the gain: the record of the victim is absent from the response, the row of the victim is unchanged, the file is unread, the local listener received no request, or the location header holds no foreign host. Assert the outcome, and not the presence of a guard.

Turn on each guard that the test environment turns off and that the production turns on, such as the check of the request token and the limit of the rate, in that test alone.

Run the test, and see that it fails with the gain of the actor in its output. Delete the candidate when the test passes, find the line that stops the attack, and keep the candidate and that line for the report.

When no test reaches the candidate, such as a fact of the production configuration, a secret in the history or two requests at one time, prove it with a command and its output: the line of the configuration file, the result of `gitleaks`, or a script in `$TMPDIR/hod-security/` that sends two requests at one time to the application started with the command of `PROJECT.md`. Keep the command and its output for the report.

Move the candidate to the list `not proven` with the exact step that you could not do, when no test and no command proves it, such as a step that needs the production server, a secret or the network. Report no item of that list as a finding.

A candidate that a test or a command proves is a finding.

## 13. Give each finding its severity

- `critical`: a guest or a user with an account that each person can create runs code or a command on the server, reads a secret of the server, reads or writes the records of each actor, or takes the account of a different actor.
- `high`: an actor reads or writes a record of one different actor, takes a right that the actor does not have, makes the server send a request to an address of the actor, or a secret of a service sits in the history of the repository.
- `medium`: an actor exceeds a limit of this project, a secret or a personal datum reaches a log or a response, or the configuration lets a page of a different origin read or send with the session.

Order the list from `critical` to `medium`.

## 14. Repair each finding that the user selects

Ask the user with the question tool of the client which findings to repair. Give each finding as an option, with its severity and its gain.

Repair each selected finding at the entry point or at the sink, with the rule of "rules-code-security" or of "rules-code-boundaries" that it breaks. Run its test, and see that it passes. Keep the test.

Move the test of each finding that the user did not select to `$TMPDIR/hod-security/tests/`, thus the suite passes and the proof stays.

Write no change to the history of the repository for a secret in it. Tell the user to rotate the secret, and name the service.

## 15. Run each check

Call the Skill tool with "rules-code-tooling".

Run each check that it names after your last change.

## 16. Report

Give the list of section 13, one finding per line, in this form: `<severity> | <actor> | <entry point> | <input> | <gain> | <path of the value> | <skill> §<section> | <test or command> | <result before the repair> | <result after the repair, or the path of the moved test>`. Write `none` when the list is empty, and write no finding to fill it.

Give each candidate that you deleted in section 11 or in section 12, with the line that stops the attack.

Give the list `not proven`, with the step that you could not do for each item.

Give the path of the map of section 2, and the number of entry points of table 1 and of sinks of table 2 that you read.

Tell the user to run `hod task-project-health` for the advisories of the dependencies, and `hod task-code-review` for each rule that the code breaks without a gain.

Say that you used no secret and wrote no secret into a file or into this report. Say that you made no commit.
