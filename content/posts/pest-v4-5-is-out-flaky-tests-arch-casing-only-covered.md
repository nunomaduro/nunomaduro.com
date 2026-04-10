---
id: 7
title: Pest v4.5 is out - flaky tests, arch casing, and --only-covered
slug: pest-v4-5-is-out-flaky-tests-arch-casing-only-covered
description: Pest v4.5 introduces flaky test support with automatic retries, a toBeCasedCorrectly arch expectation for PSR-4 compliance, and a --only-covered option to clean up coverage reports.
published_at: April 10, 2026
duration: 2 min
type: Release
---

pest v4.5 is out.. and this one's got a feature i've been wanting for a while - flaky test support

you know those tests that fail randomly - maybe they hit an external API, maybe there's a race condition, maybe the CI gods just aren't on your side.. now you can mark them as flaky, and pest will retry them automatically before reporting a failure

## flaky tests

just chain `->flaky()` on any test that's known to be unreliable:

```php
it('retries flaky operations', function () {
    // flaky code..
})->flaky(tries: 3);
```

pest will run the test up to 3 times - if it passes on any attempt, it counts as a pass.. no more re-running your entire suite because one test decided to be dramatic

and if you want to run just the flaky ones:

```bash
./vendor/bin/pest --flaky
```

this is great for isolating and monitoring your unreliable tests separately from the rest of your suite

## toBeCasedCorrectly

new arch expectation - `toBeCasedCorrectly` checks PSR-4 compliance between your class names and file paths:

```php
arch('app')
    ->expect('App')
    ->toBeCasedCorrectly();
```

if you have a class `GitHubService` in a file called `GithubService.php`, this will catch it.. it's a small thing, but PSR-4 casing mismatches can cause real issues - especially when deploying from macOS (case-insensitive) to Linux (case-sensitive)

## --only-covered

new CLI option for coverage reports - `--only-covered` hides files with 0% coverage from the output:

```bash
./vendor/bin/pest --coverage --only-covered
```

your coverage report now only shows files that actually have tests.. no more scrolling through walls of 0.0% lines to find the ones that matter

## bug fixes

this release also includes dozens of bug fixes - better `--parallel` and `--teamcity` support, fixed `toUseTrait` for inherited and nested traits, unicode support in `--filter`, dataset named parameters, and more

huge thanks to all the contributors who made this release happen

## get it

```bash
composer update pestphp/pest
```
