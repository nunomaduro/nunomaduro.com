---
id: 4
title: Short Closures RFC explained
slug: short_closures_rfc_explained
description: PHP 8.2 is scheduled to be released on the 25th of Nov, 2021. Of course, meanwhile, you may test the new features, syntax changes, and other improvements in your local environment.
published_at: July 20, 2022
duration: 4 min
type: Tutorial
---

As you may have noticed, the ["PHP RFC: Short Closures 2.0"](https://wiki.php.net/rfc/auto-capture-closure)'s status is "In Voting". This article is not about if this proposal should be accepted or not, but rather about the reasoning behind the syntax we've chosen, and how this feature was implemented from the fork until the pull request.

PHP 7.4 has introduced one-line/arrow short closures. And to make it simply, this pull request simply adds the possibility of those to be multi-line.

```diff
$users = [/** */];
$guestsIds = [/** */];
$repository = /** */;

- $guests = array_filter($users, function ($user) use ($guestsIds) {
+ $guests = array_filter($users, fn ($user) {
    $guest = $repository->findByUserId($user->id);

    return $guest !== null && in_array($guest->id, $guestsIds);
});
```

Just like every new proposal for the PHP language, the addition of this feature to PHP 8.2 is currently being discussed by the internals mailing list. But in short, the advantages are:

- Multi-line short closures don't require the `use` keyword to be able to access data from the outer scope.
- Also, `fn (/** */)` { is just shorter and simpler than `function (/** */) use (/** */) {`.

Programmers are opinionated, therefore you will find people that see themselves using this proposal, and others don't. This article is not about if this proposal should be accepted or not, but rather about the design/reasoning behind the syntax, and how this feature was implemented from the fork until the pull request.

## Design

Every programming language tries to be consistent. A programming symbol, like `return` or `class`, should mean only one, and one thing. Because of that, it's crucial to think about the visual consistency in the future of PHP "shorter" functions/methods.

Because eventually functions/methods may be using  `=>`  and `fn` symbols, let's try to give meaning to those symbols:

- `=>` means "arrow": a symbol that allows to define one-line return expressions. It's already in use by match expressions, and one-line/arrow short closures:

```php
$message = match ($statusCode) {
    200 => null,
    400 => 'not found',
};

$closure = fn () => 'foo';
```

- `fn` means "short": a symbol that makes functions/methods inherit scope. It's already in use by one-line/arrow short closures:

```php
$foo = 'foo';
$closure = fn () => $foo;
```

Now, let's try to apply this syntax to a possible future of functions/methods to see if we can make things are visual consistent:

```php
// Global or namespaced functions scopeless
function foo() {} // function
function foo() => /** */; // arrow function

// Global or namespaced functions with access to the parent scope
fn foo() {} // short function
fn foo() => /** */; // short arrow function

// Closures scopeless
$foo = function () {}; // closure
$foo = function () => /** */; // arrow closure

// Closures with access to the parent scope
$foo = fn() {}; // short closure
$foo = fn() => /** */; // short arrow closure

class Foo {
    // Methods scopeless
    function foo() {} // method
    function foo() => /** */; // arrow method

    // Methods with access to the parent scope
    fn foo() {} // short method
    fn foo() => /** */; // short arrow method
}
```

Yes — this snippet contains "features" that don't exist on PHP. And yes, maybe they will never exist in the future. Still, it's nice to see that if we give different meaning to => and fn syntax, we can have one line functions that don't inherit scope, or multi line functions that inherit scope. Things actually make sense, and things are consistent.

Now, following this reasoning, our multi-line short closures must look like this:

```php
$closure = fn () {
    return 'foo';
};
```

## Implementation

Contributing to PHP's core development is easier than you think. You don't believe me? Let's see together how this feature can be implemented from the fork until the pull request.

1. First, lets start by forking the php-src repository and get the source code:

```bash
cd <side-projects-directory>
git clone https://github.com/{your-github-username}/php-src
cd php-src
```

2. Next, lets build PHP :

```bash
./buildconf
./configure --without-iconv --disable-fileinfo
make -j12
```

> For faster builds, replace the `12` by the number of CPU cores you have available.

3. Of course, lets create a new branch where your work will go, e.g.:

```bash
git checkout -b feat/multi-line-short-closures
```

4. Now, lets update the code, and run the associated tests:

```bash
make TEST_PHP_ARGS=-j4 test TESTS=Zend/tests/arrow-functions
```

Keep in mind that typically when implementing a "simple" feature in PHP, there are 3 elements that need to be updated: Lexer, Parser, and the Compiler. And, those elements exist within the files below:

> Sometimes it can be more complicated than this.

```
zend_language_scanner.lc
zend_language_parser.y
zend_compiler.c
```

### Lexer: The dictionary that associates syntax/symbols to tokens.

The syntax/symbols is the PHP source code in the way you know it. And the other hand, tokens are the internal representation of that code in the PHP internal source code.

For our multi-line short closures, the PHP tokens we need are already known by the Lexer, thanks to the amazing work by [@nikita_ppv](https://twitter.com/nikita_ppv) in one-line/arrow short closures.

But let's imagine that the `fn` aka `T_FN` aka was missing in the language: In this case, we would have to update both `zend_language_scanner.l`, and `zend_language_parser.y` like so:

```diff
...
+<ST_IN_SCRIPTING>"fn" {
+	RETURN_TOKEN_WITH_IDENT(T_FN);
+}
...

```
<center> zend_language_scanner.lc </center>

```diff
...
+%token <ident> T_FN            "'fn'"
...
```
<center> zend_language_parser.y </center>

By updating those two files, we are instruct the Lexer that `fn` should be represented internally in PHP as the `T_FN` token.

### Parser: Creates abstract syntax trees (ASTs) from tokens.

Of course, the token `T_FN` by its own means nothing. To create something like multi-line short closures, we need to have multiple combinations of tokens all together - and those combinations of tokens are called abstract syntax trees (ASTs).

And Abstract Syntax Tree (AST) - the intermediary structure - makes our compiler, the third element, understand how to generate OPCodes aka machine code from your PHP code.

```
fn '(' parameter_list ')' '{' inner_statement_list '}'
```

Keep in mind, that `fn`, `parameter_list`, `inner_statement_list`, are already existing ASTs. As example, the `fn` AST looks like so:

```
...
fn:
	T_FN { $$ = CG(zend_lineno); }
;
...
```

For having multi-line short closures, the following combination of ASTs needs to happen:

```diff
+fn returns_ref backup_doc_comment '(' parameter_list ')' return_type backup_fn_flags '{' inner_statement_list '}' backup_fn_flags
```

> Of course, because of "phpdocs", "return types", and other features that its not worth to talk about in this article, a creating an combination of ASTs can be more complicated than this.

Now, let's add this new combination of ASTs on the file `zend_language_parser.y`, just next to the combination of ASTs that allows to create one-line/arrow short closures:

```diff
fn returns_ref backup_doc_comment '(' parameter_list ')' return_type
T_DOUBLE_ARROW backup_fn_flags backup_lex_pos expr backup_fn_flags

{
  $$ = zend_ast_create_decl(
    ZEND_AST_ARROW_FUNC,
    $2 | $12,
    $1,
    $3,
    zend_string_init("{closure}", sizeof("{closure}") - 1, 0),
    $5,
    NULL,
    zend_ast_create(ZEND_AST_RETURN, $11),
    $7,
    NULL
  );
  
  ((zend_ast_decl *) $$)->lex_pos = $10;
  CG(extra_fn_flags) = $9;
 }
 
+ fn returns_ref backup_doc_comment '(' parameter_list ')' return_type
+ backup_fn_flags '{' inner_statement_list '}' backup_fn_flags
+
+ {
+   $$ = zend_ast_create_decl(
+     ZEND_AST_ARROW_FUNC,
+     $2 | $12,
+     $1,
+     $3,
+	  zend_string_init("{closure}", sizeof("{closure}") - 1, 0),
+	  $5,
+	  NULL,
+	  $10,
+	  $7,
+	  NULL
+  );
+  CG(extra_fn_flags) = $8;
+ }
```

Both one line and multi line create the same `ZEND_AST_ARROW_FUNC` AST, but the multi line actually accepts a `inner_statement_list aka multiple expressions instead of an single line return expression.


## Compiler: Transforms ASTs in OPCodes

Finally, because the `ZEND_AST_ARROW_FUNC` AST already existed, nothing really needs to be done on the compiler. Yet, the concept is pretty simple: The file `zend_compiler.c` has a huge swich that receive the different ASTs. Each AST will have its its own `zend_compile*` function that it's goal is just compile those ASTs into OPcodes aka machine code:

```c
void zend_compile_expr(znode *result, zend_ast *ast) /* {{{ */
{
	switch (ast->kind) {
    	// ...
		case ZEND_AST_ARROW_FUNC:
			zend_compile_func_decl(result, ast, 0);
			return;
        // ...
	}
}
```

Because the content inside `zend_compile_func_decl` can be too much for this article, lets leave it for another day.

Finally, lets update the test suite. And for that, lets add the same tests that were made for one-line arrow short closures:

```
--TEST--
Basic arrow function functionality check
--FILE--
<?php

+ // No return value
+ $foo = fn() {};
+ var_dump($foo());

// Return value
$foo = fn() => 1;
var_dump($foo());

+ $foo = fn() { return 2; };
+ var_dump($foo());

$foo = fn($x) => $x;
var_dump($foo(3));

+ $foo = fn($x) { return $x; };
+ var_dump($foo(4));

$foo = fn($x, $y) => $x + $y;
var_dump($foo(4, 1));

+ $foo = fn($x, $y) { return $x + $y; };
+ var_dump($foo(5, 1));

+ // ... and 100 more tests
```

5. Push the branch of your fork on GitHub and create a pull request.

```bash
git add *
git commit -m "Adds multi line short closures"
git push
```

That's it! Remember, you don't have to change the world in your very first pull request to the PHP source code. You can fix a typo, or improve the `Readme.md`.
