---
title: Everything New in PHP 8.6
description: Everything new in PHP 8.6, ordered from the coolest feature to the smallest detail, with examples and the date of the release.
published_at: October 31, 2026
---

PHP 8.6 ships on November 19, 2026. The headline is partial function application, and it changes how every PHP developer writes callbacks.

I compiled the `PHP-8.5` and `PHP-8.6` branches of `php-src` and diffed them, so everything below comes from the source, not from a list of RFCs. The feature freeze passed in September, so the list won't change before the release.

The features are in order: the ones I'm most excited about first, the small details last.

## The timeline

| Milestone | Date |
|---|---|
| RC5 | November 5, 2026 |
| **PHP 8.6.0** | **November 19, 2026** |

The dates come from the [PHP 8.6 release page](https://wiki.php.net/todo/php86) of the PHP wiki.

## 1. Partial function application

This is the one. You write a function call, put `?` where an argument is missing, and PHP gives you back a closure that takes that argument.

```php
$startsWithAdmin = str_starts_with(?, 'admin.');

array_filter(['admin.users', 'web.home', 'admin.posts'], $startsWithAdmin);
// ['admin.users', 'admin.posts']
```

Now think about how often you write `fn ($value) => something($value, 10)` in an `array_map()` or a collection. That whole closure becomes the call itself:

```php
// Before
$posts->map(fn (string $title) => Str::limit($title, 10));

// PHP 8.6
$posts->map(Str::limit(?, 10));
```

The result is what you expect:

```php
array(2) {
  [0]=> string(13) "PHP 8.6 is..."
  [1]=> string(13) "Partial fu..."
}
```

There are three placeholders:

- `?` takes the place of exactly one argument.
- `...` takes the place of every remaining argument.
- A named argument with `?`, such as `limit: ?`, takes the place of that parameter only.

```php
function greet(string $greeting, string $name, string $punctuation = '!') {
    return "$greeting, $name$punctuation";
}

$hi = greet('Hi', ...);
$hi('Nuno');          // "Hi, Nuno!"
$hi('Taylor', '?');   // "Hi, Taylor?"

$hey = greet(name: 'Nuno', greeting: ?);
$hey('Hey');          // "Hey, Nuno!"

Str::limit(?, end: '…', limit: 5); // named arguments in any order
```

The closure you get back is a real function with real types. Reflection shows it:

```text
Closure [ <user> static function {closure:pfa:pfa.php:13} ] {
  - Bound Variables [1] {
      Variable #0 [ $greeting ]
  }
  - Parameters [2] {
    Parameter #0 [ <required> string $name ]
    Parameter #1 [ <optional> string $punctuation = '!' ]
  }
}
```

That matters, because dependency injection containers and routers read your closures with reflection. A partial keeps the types, the defaults and the names of the original function.

A `?` always makes the parameter required, even when the original parameter has a default. The `...` placeholder keeps the defaults of the original. PHP also catches the mistakes at the call:

```text
Error: Named parameter $a overwrites previous placeholder
Error: Cannot use named placeholder for unknown or variadic parameter $c
```

### With the pipe operator

PHP 8.5 gave us the pipe operator `|>`, but each step had to be a callable with one parameter. So you wrote `fn ($x) => str_replace(' ', '-', $x)` in the middle of a pipe, and the pipe lost most of its charm. Partial application fixes that:

```php
$slug = 'Hello World'
    |> strtolower(...)
    |> str_replace(' ', '-', ?);

// "hello-world"
```

```php
$title = '  Hello PHP  '
    |> trim(...)
    |> Str::limit(?, 5)
    |> strtoupper(...);

// "HELLO..."
```

And it's free. When a partial with one placeholder sits on the right side of a pipe, OPcache compiles it into a direct function call. The opcodes of `1 |> foo(?)` are the opcodes of `foo(1)`, and no closure is created.

RFC: [Partial Function Application v2](https://wiki.php.net/rfc/partial_function_application_v2).

## 2. `Time\Duration`

PHP finally has a type for an amount of time.

```php
use Time\Duration;

$timeout = Duration::fromSeconds(90)
    ->add(Duration::fromMilliseconds(500));

$timeout->seconds;     // 90
$timeout->nanoseconds; // 500000000

$meeting = Duration::fromIso8601DurationString('PT1H30M');
$meeting->seconds;     // 5400

Duration::compare($meeting, Duration::fromMinutes(90)); // 0
Duration::fromHours(2)->divideBy(3)->seconds;           // 2400
```

It's a `final readonly` class with three properties, `$seconds`, `$nanoseconds` and `$negative`. Each method gives you a new instance:

```php
object(Time\Duration)#4 (3) {
  ["seconds"]=> int(5400)
  ["nanoseconds"]=> int(0)
  ["negative"]=> bool(false)
}
```

The constructors are `fromHours()`, `fromMinutes()`, `fromSeconds()`, `fromMilliseconds()`, `fromMicroseconds()`, `fromNanoseconds()` and `fromIso8601DurationString()`. The operations are `add()`, `sub()`, `multiplyBy()`, `divideBy()`, `negate()`, `absolute()` and `compare()`.

Look at every `int $seconds` and `int $timeoutInMs` in your code: the cache TTL, the HTTP client timeout, the queue retry delay, the lock duration. The parameter name tells you the unit, and nothing checks it. A `Duration` carries its unit. I expect the libraries you use to accept it next to `int` and `DateInterval` in each place that takes a TTL or a timeout.

PHP itself already uses it. The new polling API in section 10 takes a `Duration` as its timeout, and an `int` is a `TypeError`:

```text
Io\Poll\Context::wait(): Argument #1 ($timeout) must be of type ?Time\Duration, int given
```

RFC: [Duration class](https://wiki.php.net/rfc/duration_class).

## 3. `clamp()`

You've written `max($min, min($max, $value))` a hundred times, and you've read it wrong at least once.

```php
clamp(150, 1, 100);     // 100
clamp(-5, 1, 100);      // 1
clamp(42, 1, 100);      // 42
clamp(1.5, 0.0, 1.0);   // 1.0
clamp('m', 'a', 'f');   // "f"
```

The classic use is the size of a page that a user sends you:

```php
$perPage = clamp((int) ($_GET['per_page'] ?? 15), 1, 100);
```

It works with any value that PHP can compare, strings included. A `$min` greater than `$max` is a bug, and PHP says so:

```text
clamp(): Argument #2 ($min) must be smaller than or equal to argument #3 ($max)
```

RFC: [clamp v2](https://wiki.php.net/rfc/clamp_v2).

## 4. Readonly properties with a default value

A readonly property can now declare its default value in place:

```php
final class Settings
{
    public readonly int $perPage = 15;
    public readonly string $theme = 'dark';
}
```

The property stays readonly:

```php
$settings = new Settings();

$settings->perPage; // 15
$settings->perPage = 30;
// Error: Cannot modify readonly property Settings::$perPage
```

This is for the data objects that you write in each project: the DTOs, the messages, the value objects with a fixed configuration. Before, you had to move the default into the constructor only to make the property readonly.

RFC: [Readonly property defaults](https://wiki.php.net/rfc/readonly_property_defaults).

## 5. Write to an object in a constant

You can now write to the properties of an object that a constant holds:

```php
final class Config
{
    public array $items = [];
    public int $hits = 0;
}

const CONFIG = new Config();

CONFIG->items['app.name'] = 'Acme';
CONFIG->hits++;
```

```php
array(1) {
  ["app.name"]=> string(4) "Acme"
}
int(1)
```

The constant still points to the same object, and you can't replace it. Only the object changes, as it does with a `readonly` property that holds an object. The properties of an enum case stay immutable.

RFC: [Allow writes to properties of objects in constants](https://wiki.php.net/rfc/const_object_property_write).

## 6. `#[\Override]` on constants

PHP 8.3 gave us `#[\Override]` for methods, and PHP 8.5 extended it to properties. PHP 8.6 adds class constants and enum cases:

```php
interface HasTable
{
    const TABLE = 'models';
}

final class User implements HasTable
{
    #[\Override]
    const TABLE = 'users';
}
```

If the parent constant is renamed or deleted, PHP tells you at compile time:

```text
Fatal error: Post::TABLE has #[\Override] attribute, but no matching parent constant exists
```

RFC: [#[\Override] for constants](https://wiki.php.net/rfc/override_constants).

## 7. `__debugInfo()` on enums

Enums can now decide what `var_dump()`, `dump()` and `dd()` show:

```php
enum Status: string
{
    case Active = 'active';
    case Banned = 'banned';

    public function __debugInfo(): array
    {
        return ['label' => ucfirst($this->value)];
    }
}

var_dump(Status::Active);
```

```text
enum(Status::Active) (1) {
  ["label"]=>
  string(6) "Active"
}
```

RFC: [Debuggable enums](https://wiki.php.net/rfc/debugable-enums).

## 8. JSON errors that tell you where

This one is small, and you'll love it the first time a webhook payload fails to parse.

```php
json_decode('{"name": "Nuno", "age": }');
json_last_error_msg();
```

```text
Syntax error near location 1:25
```

The `JsonException` that `JSON_THROW_ON_ERROR` throws gets the same line and column. Each place that shows that message, such as the error of your HTTP client or of your request parser, now gives you the position in the payload.

## 9. Stream errors as exceptions

Since forever, `file_get_contents()` on a missing file gave you a warning and `false`. Most frameworks turn that warning into an exception with an error handler. PHP 8.6 can do it itself:

```php
$context = stream_context_create([
    'stream' => ['error_mode' => StreamErrorMode::Exception],
]);

try {
    file_get_contents('/does/not/exist.txt', context: $context);
} catch (StreamException $e) {
    $e->getMessage(); // "Failed to open stream: No such file or directory"
}
```

The modes are `StreamErrorMode::Error`, `StreamErrorMode::Exception` and `StreamErrorMode::Silent`. There's also `stream_last_errors()` and `stream_clear_errors()` to read the errors without a handler, and a `StreamErrorCode` enum so you don't compare message strings. The old behavior stays the default.

Related: `fclose()`, `file_put_contents()` and `copy()` now return `false` when the flush or the close fails, such as a full disk at the last write.

RFC: [Stream errors](https://wiki.php.net/rfc/stream_errors).

## 10. A real polling API: `Io\Poll`

This is the foundation for async PHP. It's `epoll` on Linux and `kqueue` on macOS, behind one object API, with no extension to install:

```php
use Io\Poll\Context;
use Io\Poll\Event;
use Time\Duration;

[$a, $b] = stream_socket_pair(STREAM_PF_UNIX, STREAM_SOCK_STREAM, STREAM_IPPROTO_IP);

$poll = new Context();
$poll->add(new StreamPollHandle($b), [Event::Read], data: 'worker-1');

fwrite($a, 'ping');

foreach ($poll->wait(Duration::fromSeconds(1)) as $watcher) {
    $watcher->getData(); // "worker-1"
    fread($b, 4);        // "ping"
}
```

You won't write this in a controller. It's for the people who write event loops, such as ReactPHP, Amp and Revolt, which today fall back to `stream_select()` or need an extension for `epoll`. Everything built on them, such as WebSocket servers and HTTP clients, can get it for free. The backends are `Auto`, `Poll`, `Epoll`, `Kqueue`, `EventPorts` and `WSAPoll`.

RFC: [Poll API](https://wiki.php.net/rfc/poll_api).

## 11. `SortDirection`

A global enum with two cases:

```php
enum SortDirection
{
    case Ascending;
    case Descending;
}
```

No function of PHP 8.6 takes it yet. It's there so your packages stop inventing their own `'asc'` and `'desc'` strings:

```php
function orderBy(string $column, SortDirection $direction = SortDirection::Ascending): string
{
    return $column . ' ' . match ($direction) {
        SortDirection::Ascending => 'asc',
        SortDirection::Descending => 'desc',
    };
}

orderBy('created_at', SortDirection::Descending); // "created_at desc"
```

I'd like to see `->orderBy('created_at', SortDirection::Descending)` in every query builder.

RFC: [Sort direction enum](https://wiki.php.net/rfc/sort_direction_enum).

## 12. Build a URI

PHP 8.5 added the `Uri` extension to parse URIs. PHP 8.6 adds the other half, a builder:

```php
$uri = (new Uri\Rfc3986\UriBuilder())
    ->setScheme('https')
    ->setHost('nunomaduro.com')
    ->setPath('/posts')
    ->setQuery('page=2')
    ->build();

$uri->toString(); // "https://nunomaduro.com/posts?page=2"
```

There's also `Uri\WhatWg\UrlBuilder` for the browser rules, `getUriType()`, `getHostType()` to know if a host is an IPv4, an IPv6 or a domain, and `Uri\WhatWg\url_percent_encode()`.

RFC: [URI follow-up](https://wiki.php.net/rfc/uri_followup).

## 13. Faster without a change to your code

These come from the "Performance Improvements" section of `UPGRADING`.

- `array_map()` with a first-class callable or a partial, such as `array_map(strtoupper(...), $names)`, is compiled into a `foreach` loop. No closure is created, and internal code doesn't call back into userland.
- `printf()` with only `%s` and `%d` is compiled into string interpolation.
- Stateless closures are cached. A `static` closure that captures nothing is created once and reused. See the example below.
- Arguments are passed faster to known constructors, such as `new self()`.
- `json_encode()` is faster on arrays and objects, and on `JSON_PRETTY_PRINT`.
- `array_sum()` and `array_product()` on integer arrays, `array_unshift()`, `array_walk()`, `array_intersect()`, `str_pad()`, `str_split()`, `mb_str_pad()` and `pathinfo()` with one component are faster.
- Deep recursion is faster when the VM allocates new stack pages.
- The JIT now works on ZTS builds on Apple Silicon, and ZTS builds are faster. That's the build that FrankenPHP uses.

You can see the closure cache from userland:

```php
function make() { return static fn () => 1; }

var_dump(make() === make());
// bool(true)
```

The [RFC](https://wiki.php.net/rfc/closure-optimizations) measured about 3% on a real application template, but that number includes a second optimization that wasn't merged. Take it as a direction, not a promise.

## 14. The small things you'll use

- **`trim()` removes the form feed.** `"\f"` joins the default characters of `trim()`, `ltrim()` and `rtrim()`.
- **`ARRAY_FILTER_USE_VALUE`** is the new name for the default mode of `array_filter()`, so you can write the mode explicitly.
- **`WeakMap` references.** `$ref = &$map[$object]` works when the key isn't there yet.
- **`pack()` and `unpack()` take `<` and `>`** for little and big endian on the integer, float and double codes: `unpack('s<value', $bytes)`.
- **`ReflectionProperty::isReadable()` and `isWritable()`** tell you if the current scope can read or write a property, asymmetric visibility and hooks included. Serializers and form libraries need this.
- **`ReflectionParameter::getDocComment()`** reads a doc comment on a parameter.
- **`ReflectionAttribute::getShortName()`**, `getNamespaceName()` and `inNamespace()`.
- **`mysqli::quote_string()`** quotes and escapes a string in one call.
- **`grapheme_strrev()`** reverses a string by grapheme, so `"👋🏽"` stays one emoji. It's in `intl`.
- **`ini_get_all()`** gives a `builtin_default_value` for each directive.
- **The built-in server** (`php -S`) accepts the HTTP `QUERY` method and no longer reflects the `Host` header.
- **`error_include_args`**, a new INI option, shows the arguments of a function in its error message. It's off by default.
- **OpenSSL** gets TLS session resumption, external PSK and TLS 1.3 early data for streams.
- **Sodium** gets the post-quantum ML-KEM768 and X-Wing KEMs (with libsodium 1.0.22), SHAKE and TurboSHAKE, and IP address encryption.
- **Curl** gets `CURLOPT_SEEKFUNCTION`, so a streamed upload can rewind on a redirect.
- **Intl** gets `IntlNumberRangeFormatter`, for "€3 – €5" in each locale.
- **Unicode 18.0** in `mbstring`.

## 15. What can break

Read this section before you upgrade production.

**The session defaults are secure now.** `session.use_strict_mode` is `1`, `session.cookie_httponly` is `1`, and `session.cookie_samesite` is `"Lax"`. If your framework has its own session layer, or passes its own session options, check its session configuration. An app that calls `session_start()` with the defaults that reads the session cookie from JavaScript, or that expects the session cookie on a cross-site `POST`, breaks.

**A `ValueError` where there was a warning.** A lot of functions now throw for a value that was silently wrong: a NUL byte in a path given to `file_exists()`, `is_file()`, `filesize()` or `stat()`, a negative `sleep()`, an invalid mode for `array_filter()`, `pathinfo()` or `scandir()`. If your code passed garbage before, it now gets an exception.

**`preg_grep()` returns `false`** on a PCRE error, such as bad UTF-8 with `/u`, instead of a partial array.

**`trim()` removes `"\f"`.** If you kept a form feed at the end of a string on purpose, use `trim($value, " \n\r\t\v\0")`.

**`SplFileObject`** changed how `next()`, `fgets()` and `current()` share the current line. If you read files line by line with it, run your tests.

**`base_convert()`, `bindec()`, `hexdec()` and `octdec()`** emit a notice when the result loses precision.

**`session_encode()`** returns an empty string for an empty session, not `false`.

**A hard out of memory** calls `abort()` and exits with the code 134, not 1. Your process monitor sees a different code.

**A `foreach` by reference** can now visit an element that it skipped before, after the array is compacted. This is an edge case, but it's in the list.

The full list is in section 1 of `UPGRADING` in `php-src`. It's long because most of it is one extension at a time: GD, GMP, Intl, Phar, SOAP, Sockets, Zip, Zlib.

## 16. What's deprecated

A deprecation is not an error. Your code runs, and you get an `E_DEPRECATED` notice that tells you what to change before PHP 9. Here are a few:

```text
Deprecated: Returning a value from a constructor is deprecated
Deprecated: Returning from a finally block is deprecated
Deprecated: Function is_integer() is deprecated since 8.6, use is_int() instead
Deprecated: Function is_double() is deprecated since 8.6, use is_float() instead
Deprecated: Function doubleval() is deprecated since 8.6, use floatval() instead
Deprecated: Function spl_object_hash() is deprecated since 8.6, consider using spl_object_id() instead
```

The language:

- A `return` with a value in `__construct()` or `__destruct()`, and a constructor or a destructor that is a generator.
- A `return` in a `finally` block.
- `let` and `is` as the name of a class, an interface, a trait, an enum, a function or a constant. Those two words are reserved for the future, so don't name a class `Is`.
- `readonly` as the name of a function, `namespace` as the name of a class constant, `_` as the name of a constant.
- A third argument to `define()`.
- `is_a()` and `is_subclass_of()` with a string when `$allow_string` is `false`.
- `?array` as the return type of `__debugInfo()`. Write `array`.

The functions:

- `is_integer()`, `is_long()`, `is_double()` and `doubleval()`. Use `is_int()`, `is_float()` and `floatval()`.
- `spl_object_hash()`. Use `spl_object_id()`. Check your packages for this one.
- `spl_classes()`, `metaphone()`, `strcoll()` and the `SORT_LOCALE_STRING` flag.
- An object passed to `array_walk()`, `array_walk_recursive()`, `http_build_query()` or `mb_convert_variables()`. Call `get_object_vars()` first.
- `mysqli_get_charset()` and `mysqli_stmt_init()`.
- The CSV methods of `SplFileObject`, and the sort methods of `ArrayIterator`.
- `mbregex`, the `mb_ereg*()` functions, because the Oniguruma library has no maintainer.
- More than 16 filters in a `php://filter` URL.
- `ReflectionProperty::setValue()` with an object of the wrong class, and `ReflectionMethod::invoke()` with an object for a static method.
- A session handler without `create_sid()` and `validateId()`.

## How to try it

You don't have to wait for November 19. Compile the branch yourself:

```bash
git clone --depth 1 --branch PHP-8.6 https://github.com/php/php-src.git
cd php-src
./buildconf --force
./configure --disable-all --enable-cli --enable-mbstring --disable-mbregex --enable-tokenizer --enable-ctype --enable-filter
make -j8

sapi/cli/php -r 'var_dump(clamp(150, 1, 100));'
// int(100)
```

That's PHP 8.6. It's an exciting release, and the PHP team did a great job on it.

The past month was about security vulnerabilities, and people are still talking about them. I hope that conversation doesn't bury this release. It deserves your attention.

PHP is a great language. [dev-null.sh](https://dev-null.sh) and all the hype around it showed that again. I love PHP, and I love this community.
