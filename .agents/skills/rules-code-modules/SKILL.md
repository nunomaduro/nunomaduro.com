---
name: rules-code-modules
description: "The rules for the shape of a module and its interface. Use when you add a class, a module, a service, a layer, an interface, an abstract class or an adapter, when you split or merge modules, when a module only passes each call to a different module, when you choose what a module shows to its callers, when you write an interface for a provider, and when you write a type that holds a different item."
---

# Code Modules

Obey each rule of this file each time that you write a module and each time that you change the interface of a module.

## 1. Give each module one secret

Write in one sentence the decision that the module keeps from its callers, such as the format of a file, the provider of a payment or the order of the steps of an import. Write no module when you cannot write that sentence.

Keep each fact of that decision inside the module. A caller that must know the format, the provider or the order to call the module is a fault of the module.

## 2. Move a sequence of calls into the module

When two callers or more make the same sequence of calls on a module, give the module one operation that makes the sequence. Let each caller finish its work with one call.

Show a caller no operation that only one step of the sequence needs.

## 3. Write a new module at the third copy

Copy the code once when a second caller needs it. Write a new module when a third caller needs the same code, or when the code holds a secret of section 1.

Write no module that only passes each call to a different module. Write no type and no function that gives the same operations as the item that it holds. Merge each such item into its caller.

## 4. Write an interface only at the edge of the program or for two production implementations

Write an interface for each system outside the program, such as an HTTP API, the clock, the mail or the file system, thus a test gives a fake of that system.

Write no interface inside the program for one production implementation. Write the class, and write the interface when a second production implementation ships.

## 5. Point each dependency in one direction

Let a module of the domain call no module of the delivery, such as a controller, a command or a page. Give a module each dependency through its constructor or its parameters.

## 6. Report

Name each module that your change adds, and give the sentence of section 1 for it.

Name each operation that replaced a sequence of calls, and name its callers.

Name each interface that your change adds, and name the system outside the program or the two production implementations that it serves.

Name each module that only passed each call, and say that you deleted it or merged it.

Write this report in the last message of your answer. Write no sentence for a rule of this file that your work did not touch.
