---
id: 5
title: a Laravel Pint agent format - my proposal
slug: laravel_pint_agent_format
description: Laravel Pint can now automatically detect when it's being run by AI coding agents like Claude Code or OpenCode, and switches to a structured JSON output format that's optimized for machine parsing.
published_at: January 4, 2026
duration: 2 min
type: Tutorial
---

i've been super into [OpenCode](https://opencode.ai) lately.. and something i've noticed - when it runs Pint, it gets back colorful human output and has to guess what happened..

so i've opened [a PR](https://github.com/laravel/pint/pull/415) to add `--format agent` to Pint - no guarantees it gets merged, but here's the idea..

this format outputs clean JSON - and auto-activates when Pint detects it's running inside Claude Code or OpenCode..

## the output

```json
{"status":"pass"}
```

or when there are issues:

```json
{"status":"fixed","files":[{"path":"app/Models/User.php","fixers":["binary_operator_spaces","no_unused_imports"]}]}
```

three status values: `pass` (clean), `fail` (issues found in `--test` mode), `fixed` (issues auto-corrected)..

## why this matters

AI agents burn tokens parsing human-formatted output and guessing what happened.. with structured JSON, your agent knows exactly which files failed and which fixers were applied - it can take targeted action instead of re-running commands or asking clarifying questions..

## how it works

Claude Code sets `CLAUDECODE`, OpenCode sets `OPENCODE`.. Pint checks for these:

```php
public static function runningInAgent(): bool
{
    return (bool) getenv('OPENCODE') || (bool) getenv('CLAUDECODE');
}
```

for other tools or custom integrations:

```bash
./vendor/bin/pint --format agent
./vendor/bin/pint --test --format agent  # report without fixing
```


