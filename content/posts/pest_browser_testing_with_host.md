---
id: 6
title: testing subdomain applications with Pest browser testing
slug: pest_browser_testing_with_host
description: Pest browser testing now supports the withHost method, making it easy to test subdomain applications by configuring the host globally or per-test.
published_at: January 8, 2026
duration: 2 min
type: Tutorial
---

you probably noticed this was missing on Pest's browser testing - a way of testing subdomains in your laravel applications..

if you're using Laravel Sail or building multi-tenant apps with subdomain routing, browser tests just didn't work.. the server was hardcoded to `127.0.0.1`, so subdomains couldn't resolve properly..

well, we've just merged `withHost` - and it works in two ways depending on what you need..

## global configuration

when all your browser tests need the same host, configure it once in `Pest.php`:

```php
pest()->browser()->withHost('tenant.localhost');
```

this changes the server binding for your entire test suite..

## per-test configuration

sometimes you need different hosts for different tests.. maybe you're testing tenant isolation or verifying that different subdomains serve different content.. just chain `withHost` on your visit:

```php
$page = visit('/dashboard')->withHost('tenant.localhost');

$page->assertSee('Welcome to Tenant Dashboard');
```

## why this matters

this was one of the most requested fixes - subdomain routing is everywhere in laravel apps.. multi-tenant saas, workspace-based apps, region-specific subdomains..

before this, you'd hack around with `/etc/hosts` or just skip browser testing those routes entirely.. now it's one method call and your subdomain routes are testable like everything else..
