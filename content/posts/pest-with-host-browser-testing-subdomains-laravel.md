---
id: 6
title: Pest's withHost - browser testing subdomains in Laravel
slug: pest-with-host-browser-testing-subdomains-laravel
description: Pest browser testing now has withHost, finally fixing subdomain testing for Laravel Sail and multi-tenant apps.
published_at: January 8, 2026
duration: 2 min
type: Tutorial
---

you probably noticed this was missing on Pest's browser testing - a way of testing subdomains in your laravel applications..

if you're using Laravel Sail or building multi-tenant apps with subdomain routing, browser tests just didn't work.. the server was hardcoded to `127.0.0.1`, so subdomains couldn't resolve properly..

well, we've just merged `withHost` - and it fixes this..

## the problem

say you have subdomain routing in your laravel app:

```php
Route::domain('{tenant}.localhost')->group(function () {
    Route::get('/dashboard', function (string $tenant) {
        return view('dashboard', ['tenant' => $tenant]);
    });
});
```

before `withHost`, there was no way to browser test this.. the test would just hit `127.0.0.1` and your subdomain route wouldn't match..

## the fix

now you can set the host per-test:

```php
it('shows the tenant dashboard', function () {
    $page = visit('/dashboard')->withHost('acme.localhost');

    $page->assertSee('Welcome to Acme');
});
```

or configure it globally in `Pest.php` when all tests use the same host:

```php
pest()->browser()->withHost('acme.localhost');
```

## laravel sail

if you're running Laravel Sail, just point to your sail hostname:

```php
pest()->browser()->withHost('laravel.test');
```

or for subdomain apps on sail:

```php
pest()->browser()->withHost('acme.laravel.test');
```

## testing multiple tenants

the real power is testing tenant isolation - making sure each tenant only sees their own data:

```php
it('isolates tenant data', function () {
    visit('/dashboard')
        ->withHost('acme.localhost')
        ->assertSee('Acme Dashboard')
        ->assertDontSee('Globex');

    visit('/dashboard')
        ->withHost('globex.localhost')
        ->assertSee('Globex Dashboard')
        ->assertDontSee('Acme');
});
```

this was one of the most requested fixes.. multi-tenant saas, workspace-based apps, region-specific subdomains - all testable now with one method call..

## get it

```bash
composer update pestphp/pest-plugin-browser
```
