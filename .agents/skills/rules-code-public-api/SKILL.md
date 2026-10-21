---
name: rules-code-public-api
description: "The rules for the public API of a package and the visibility of each item. Use when you write a new type, a new method or a new constructor, when you change the visibility of a type or of a member, when a caller outside this project can reach a type or a member, when you add a parameter to a signature that a caller outside this repository calls, when you decide which file holds the public API, and when the code carries `@internal`, `internal`, `pub(crate)`, `private_constant`, `__all__` or a directory `internal/`."
---

# Code Public API

Obey each rule of this file each time that you write a type or a member and each time that you change a type or a member.

## 1. Mark each item that is not the public API

Read `.hod/PROJECT.md` before you write a new type and before you change the visibility of a type. Take two facts from that file: whether this project is a package that a different developer installs or an application, and the file that holds the public API.

Give the public API of a package one place, and mark each other type of that package with the marker of this table. A reader thus sees the boundary in the file, and a change to a type behind that boundary breaks no caller.

A new type of a package is not the public API unless `.hod/PROJECT.md` names it there, thus that type carries the marker.

| The language | The marker of a type that is not the public API |
| --- | --- |
| PHP | `@internal` |
| TypeScript | a type that the entry point of the package does not export |
| Java | a package private type |
| Kotlin | `internal` |
| C# | `internal` |
| Swift | `internal` |
| Rust | `pub(crate)` |
| Python | a name with the prefix `_`, and an `__all__` that omits that name |
| Go | a name that starts with a small letter, or the directory `internal/` |
| Ruby | `private_constant` |

Give each member the narrowest visibility that its callers need. Write each method private, and make it public at the first caller outside its own type. A member that one method of the same type calls stays private.

In a language that holds a constructor and a visibility, give the constructor the visibility `private` when one static method of the same type is the one place that builds the object, and give that method a name that says what it builds, such as `create`.

## 2. Keep the public API of a package compatible

Accept a default value in the public API of a package when you add a parameter to a signature that a caller outside this repository already calls. A new required parameter breaks that caller, and a default value keeps the version compatible.

Write no default value in a new signature of that public API. No caller outside this repository calls a new signature, thus no caller breaks.

Each other file is internal: the code of an application, and the code below the public API of a package. Call the Skill tool with "rules-code-type-safety" for each nullable and each default value, in the public API and in internal code.

## 3. Report

Call the Skill tool with "rules-code-tooling" after you change a type or a member, and run each check that it names.

Name `.hod/PROJECT.md`, say whether that file calls this project a package or an application, and name the file that holds the public API, each time that you wrote a type or changed a type.

Name each type that you marked with the marker of section 1, and name each member that you made private. Name each type that you left without that marker, and give the reason.

Name each default value that you accepted in the public API under section 2, and name the caller outside this repository that it keeps compatible.

Say that you ran the tests of this project, and give the result of that run.

Write this report in the last message of your answer. Write no sentence for a rule of this file that your work did not touch.
