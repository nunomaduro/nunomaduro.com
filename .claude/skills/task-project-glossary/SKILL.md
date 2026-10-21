---
name: task-project-glossary
description: "Keep the terms of the domain of this project in `.hod/GLOSSARY.md` and each decision about its structure in `.hod/decisions/`. Use when the user asks for a glossary, asks what a term means, uses two names for one thing, chooses a dependency, a provider, a format of the data or a public interface, or asks to record a decision of the architecture."
disable-model-invocation: true
---

# Project Glossary

Give each term of the domain one name and one meaning, and record each decision that the user makes about the structure of the project. Do each section of this file in its order.

## 1. Read the terms

Read `.hod/PROJECT.md`, `.hod/GLOSSARY.md` when it exists, the names of the models, of the tables and of the routes, and the words of the request.

List each term of the domain: each thing, each actor, each state and each operation that a user of the program names.

## 2. Find the conflicts

Find each term with two names, such as `shop` and `store` for one thing, and each name with two meanings.

Leave out each name that the framework or the language gives, such as the column of a foreign key or of a timestamp, and the name of a class of the framework.

Ask the user with the question tool of the client which name to keep for each conflict. Give the names that the code uses as the options.

## 3. Write the glossary

Write `.hod/GLOSSARY.md` with one line for each term: the term in `code font`, its meaning in one sentence, and the name in the code when it differs.

## 4. Record each decision of the user

Record each decision that the user makes in this session about a dependency, a provider, a format of the data, the shape of a table or a public interface. Record a decision of that type that the code holds when the user gives its reason.

Write it to `.hod/decisions/<number>-<name>.md` in the form of the architecture decision record of Michael Nygard: a title, then the sections `Status`, `Context`, `Decision` and `Consequences`. Name each option that the user refused in `Context`.

## 5. Report

Give the count of the terms in `.hod/GLOSSARY.md`.

Name each conflict and the name that the user kept.

Name each decision that you recorded, and give its path.

Name each place in the code that still uses a name that the user refused, and tell the user to rename it in a separate change.
