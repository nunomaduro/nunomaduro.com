---
name: rules-code-tests
description: "The rules for the tests of a change and the place of each test. Use when you write a test, when you choose the path of a test file, when the project holds an example test, when you add a behavior, a route, a controller, a page, a command, a job, a listener, a query or a permission, when you change what a function gives back, when you choose what a test asserts, when a list can be empty, when a value has a limit such as the size of a page, when a change adds or changes a page in a browser, and when you finish a change."
---

# Code Tests

Obey each rule of this file each time that you write a test and each time that you change the behavior of the code.

## 1. Give each requirement of the request a test

Write each requirement of the request in a list before you write code. A requirement is one sentence that a user can check: a page shows a value, a command writes a file, a user cannot open a record.

Write one test or more for each item of that list. A requirement without a test is not done.

Give each test the name of its requirement, such as `hides a draft post from the home page`.

## 2. Put each test in the layer of what it tests

Give each test one of three layers: unit, feature and browser. Write each new test in this layout. Move no test that your change does not touch.

An entry point is a file that receives a call from outside the program: a request over HTTP, a command of the command line and a message of a queue. Each other file of the code is not an entry point.

The entry points are each controller or handler of a request over HTTP, each command of the command line and each job or worker of a queue.

A feature test drives one entry point. Write it at the path of that entry point, under the directory of the feature tests, thus the tree of the feature tests mirrors the tree of the entry points and holds no other directory:

| Entry point | Feature test |
| --- | --- |
| `src/Http/Controllers/PostController.php` | `tests/Feature/Http/Controllers/PostControllerTest.php` |
| `src/Console/Commands/PruneCommand.php` | `tests/Feature/Console/Commands/PruneCommandTest.php` |
| `src/Jobs/SendInvoice.php` | `tests/Feature/Jobs/SendInvoiceTest.php` |
| `src/handlers/orders.ts` | `tests/feature/handlers/orders.test.ts` |
| `src/main.rs`, the command line of a Rust binary | `tests/cli.rs` |

Give one entry point one test file with the name of that entry point, such as `PostControllerTest.php`, and test each public method of it in that file. Write no file for one method.

A component that a route serves in the place of a controller is a method of the controller of its resource. Give the components of one resource one test file under the directory of the controllers: the pages `teams/index` and `teams/show` give `tests/Feature/Http/Controllers/TeamControllerTest.php`. Use that name when the project holds no controller of that resource too, and report no fault for it.

Write each test at the top level of its file. Write no group of tests, such as a `describe` block. Name the method in the name of the test, such as `stores a project with a description` and `forbids a member to destroy a project`.

Write the setup of each test in that test. Copy a short setup into the next test. Write a helper function only for a setup of more than five lines that three tests or more need.

A unit test tests one file of the code that is not an entry point, such as a model, a policy, an enum, an action and a class of `src/Support/`. Write it at the path of that file, under the directory of the unit tests, thus the tree of the unit tests mirrors the tree of the code without the entry points:

| Language | File of the code | Unit test |
| --- | --- | --- |
| PHP | `src/Policies/TeamPolicy.php` | `tests/Unit/Policies/TeamPolicyTest.php` |
| TypeScript | `src/support/money.ts` | `tests/unit/support/money.test.ts` |
| Python | `src/shop/support/money.py` | `tests/unit/support/test_money.py` |
| Rust | `src/support/money.rs` | a `#[cfg(test)] mod tests` in `src/support/money.rs` |
| Go | `support/money.go` | `support/money_test.go` |

Write a unit test for each file that is not an entry point and that holds a decision of this project, such as a branch, a calculation, a condition of a query or a rule of a policy, even when a feature test also runs it. Write no unit test for a method that only declares what the framework gives, such as a relation between two records, a cast or an accessor without a branch, because that test tests the framework. Write no unit test for an entry point, because its feature test covers it.

Give the unit tests the setup that the file needs, such as a database that each test resets for a model that reads the database.

Delete each example test that the framework or the template of the project wrote, such as `tests/Feature/ExampleTest.php`, because it tests no code of this project.

A browser test drives a page in a real browser, through the URL of the page. Write it at the path of the URL, under `tests/Browser/`: `/` gives `tests/Browser/HomeTest.php`, `/teams` gives `tests/Browser/TeamsTest.php` and `/teams/{team}/projects` gives `tests/Browser/Teams/Team/ProjectsTest.php`.

Ask the user with the question tool of the client whether this project needs browser tests, when your change adds or changes a page and the project holds no browser test. Name the package that the browser tests need. Write browser tests only after the user agrees.

## 3. Test each operation of an entry point at each edge

Write a test for each operation of each entry point that your change adds or changes, such as each method of a controller (`index`, `create`, `store`, `show`, `edit`, `update` and `destroy`) and each public action of a component that a page calls.

Write these tests for each operation:

- the operation that succeeds, and an assertion of each row and each value that it writes or shows
- each rule of the validation that refuses a wrong value, and an assertion that the refusal wrote no row
- each argument and each public property that the operation receives, with a value of the wrong type or outside its enum, even when the code holds no rule for it, and an assertion that the answer is a refusal and not an error of the server
- each actor that the operation lets in, and each actor that it stops: each role that the request names, a user of a different team or a different account, and a guest without a session
- a record that does not exist, and a record of a different parent, such as a task of a project of a different team

Assert the exact status of a refusal, such as 401, 403, 404 or 422. Assert that the refusal changed no row of the database.

## 4. Test each limit and each empty state

Write a test for the two sides of each limit. A page of 10 rows needs 10 rows on the first page and the eleventh row on the second page.

Write a test for each list that can be empty, and assert the text that the page shows for it.

Write a test for each optional value when it is absent.

## 5. Make each test fail for the wrong code

Assert the value that the requirement names, not the status alone. A test of a page asserts the text of the record, and a test of an order asserts the position of each record.

Give each record of a test a value that no other record holds, thus an assertion cannot pass on the wrong record.

Take each expected value from a literal in the test, such as `1500`, not from the calculation that the code does.

Use a test double only for a dependency outside the program: an API over HTTP, the clock, a random value, the mail and the queue. Run the database and each class of this project as it is.

Change the code that a new test covers one time, and see that the test fails. Then restore the code.

## 6. Report

Give the list of the requirements of section 1, and give the name of each test of each requirement.

Name each test file that you wrote, give its layer, and give the file of the code, the entry point or the URL that it tests.

Name each example test that you deleted.

Name each operation of each entry point, and name its tests for the success, the validation, each argument with a wrong value, each actor and the record that does not exist.

Name each limit and each empty state, and name its test.

Say whether the user asked for browser tests, and name the browser tests that you wrote.

Say that you ran the tests of this project, and give the result of that run.

Write this report in the last message of your answer. Write no sentence for a rule of this file that your work did not touch.
