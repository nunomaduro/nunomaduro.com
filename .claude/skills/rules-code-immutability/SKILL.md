---
name: rules-code-immutability
description: "The rules for the state of an object. Use when you write a new class, a new struct or a new record, when you write a constructor, when you add a property to a type that exists, when you write a setter or a method that writes a property of its own object, when a caller needs an object with one different value, when a method sorts, adds to or removes from a collection that it received, when a method gives back a collection that a property holds, when you take or give back a date or a time, when you fill a property in a step after the construction, when you decide that a different developer can extend a class of this project, and when the code carries `open`, `extends` or `clone`."
---

# Code Immutability

Obey each rule of this file each time that you write a type and each time that you change a type.

A type is a class, a struct or a record. A property is a field of that type.

## 1. Close each type to extension

Write each new class closed. A class that a different class extends carries the behaviour of each of its children, thus a method of that class gives a result that its own code does not show.

Give the class the keyword of section 3. Write no `protected` property and no `protected` method in a closed class. No class extends it, thus write each of those members `private`.

When this project needs an extension point today, write an interface: each caller takes the interface, and each implementation stays closed. Open no class for it.

## 2. Give each property one value for the life of the object

Take each value of the object as a parameter of the constructor, write that value into the property in the constructor, and mark the property read only with the keyword of section 3.

Write no property that a step after the construction fills. That property carries two states: the state before that step and the state after it. Call the Skill tool with "rules-code-type-safety" for the replacement of a property that carries two states.

## 3. Write the keyword of the language

| The language | Close a type | Freeze a property | Give a changed copy |
| --- | --- | --- | --- |
| PHP | `final class` | `readonly class`, or `public readonly` on one property | `new self(...)` |
| TypeScript | no keyword | `readonly` | `{ ...this, name: value }` |
| Java | `final class`, or a `record` | a `final` field, or a `record` | `new Name(...)` |
| Kotlin | a class without `open`, or a `data class` | `val` | `copy(name = value)` |
| C# | `sealed class`, or a `record` | `init` or `readonly` | `this with { Name = value }` |
| Swift | `final class`, or a `struct` | `let` | `Self(...)` |
| Rust | no keyword, because no type extends a `struct` | no `mut` on the binding | `Self { name: value, ..*self }` |
| Python | `@final` and `@dataclass(frozen=True)` | `@dataclass(frozen=True)` | `dataclasses.replace(self, name=value)` |
| Go | no keyword, because no type extends a struct | no keyword, and a value receiver on each method | `Name{...}` |
| Ruby | no keyword | `attr_reader`, and `freeze` at the end of `initialize` | `self.class.new(...)` |

Write no keyword where a cell of the table says `no keyword`, and obey the rule by the form of the code alone.

Read the version of the language in the manifest of the project, such as `composer.json` or `Cargo.toml`, when the keyword of the table needs a version that the project does not hold, and write the form of that version.

## 4. Change no object after its construction

Write no setter, and write no method that writes a property of its own object.

Write a method that returns a new object of the same type when a caller needs an object with one different value, and build the new object with the form of section 3. Give that method the name `with<Property>` in the case of the language, such as `withAddress` and `with_address`. Write no such method in a language whose form of section 3 gives the copy at the call site, such as `copy(name = value)` in Kotlin and `this with { Name = value }` in C#.

## 5. Change no object that you receive and no object that you give back

Copy a collection before you sort it, before you add to it and before you remove from it. The caller holds the same collection, thus a change of that collection changes the object of the caller.

Give back a copy of a collection that a property holds, or give back a form that the caller cannot write, such as a frozen array, an immutable list or a read only view.

Take the immutable form of a type that the language gives in two forms, such as `DateTimeImmutable` in the place of `DateTime` in PHP and `java.time.Instant` in the place of `java.util.Date` in Java. A method of the mutable form writes the object that each caller holds.

## 6. Stop when a tool of this project needs a mutable type

Read the manifest of the project for its tools. Three tools need a mutable type: an object relational mapper that writes a property after the construction, a container that builds a class with a setter, and a library that deserializes a type from a document.

Keep that mutable type at the boundary of that tool, and write an immutable type of this project that a constructor builds from it. The code of this project then reads the immutable type alone.

Stop when the tool gives no such boundary. Give the user the path of the file, the name of the tool and the rule of this file that the tool breaks. Ask the user to accept the mutable type, and wait for the answer.

Judge each exception on its own. An exception that the user accepted for one type gives no exception for a different type.

## 7. Report

Call the Skill tool with "rules-code-tooling" after you change a type, and run each check that it names.

Name each type that you closed, and give the keyword of section 3 that you wrote.

Name each property that you made read only.

Name each setter that you deleted, and name the method that gives a new object in its place.

Name each collection that you copied before you changed it, and name each collection that you gave back as a copy or as a read only view.

Name each mutable type of the language that you replaced, and name the immutable form that you wrote in its place.

Say that you ran the tests of this project, and give the result of that run.

Write this report in the last message of your answer. Write no sentence for a rule of this file that your work did not touch.
