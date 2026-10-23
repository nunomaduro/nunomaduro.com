---
title: So, about the PHP security release
description: What I did with my own apps when php2shell dropped, the people who shipped the fix, and why you should build like you're already going to get hacked.
published_at: October 22, 2026
---

The details about these two CVEs are everywhere, so I won't tell you about them again. This is about my thoughts on all of it, and how the day went for me.

## My morning

It was 8 AM when I woke up to a bazillion messages, literally everywhere. That scared me.

So I took down every website I had. Some went offline, some got a static page instead. Then I deleted a bunch of side projects that were lingering around for no reason. If a project isn't online, nobody can hack it.

Everything I run is on Laravel Cloud, so once the team patched the platform, I put it all back online. Props to the Laravel Cloud team.

## The people you don't see

A new PHP version isn't a `git tag`. There are build pipelines, packages for every platform, and mirrors that spread the downloads around the world. And yesterday, the whole world wanted all of it at the same time.

All of that sits on a very small group of people. Some of them aren't even paid for it.

We also saw big governments and big corporations tweet about it, complain a bit, and donate exactly zero dollars to the cause. I find that hilarious.

This needs to be fixed. Or maybe that's just how open source is. I don't even know at this point.

## The embargo

This part of the story still confuses me.

The fix had an embargo until 14:00 UTC. Chainguard shipped fixed builds before that, and [Dan Lorenc explained why](https://www.linkedin.com/posts/danlorenc_i-wanted-to-share-what-happened-with-the-share-7518756991520120832-QX2_/): they saw reports of exploits in the wild and wanted to protect people. The PHP Foundation disagreed.

But I don't buy the "protect people" part. A fixed build shows exactly what got fixed, and what got fixed tells you exactly where the hole is. Ship that early and you're handing out weapons. Not only to hackers, but to anyone with an AI model and a free afternoon.

So yeah, this whole story needs to be clarified.

## What you should do

Update PHP today: `8.5.12`, `8.4.27`, `8.3.36` or `8.2.35`. If you're on something older than `8.2`, there's no fix for you, so today is the day you upgrade.

Then the bigger thing. It's 2026. Build like you're already going to get hacked, because sooner or later you probably will.

**Collect less.** Only keep the customer data you actually need. Anything you never store can't leak.

**Encrypt everything you keep.** Not only passwords and card numbers: names, emails, addresses, photos, uploads, all of it. If someone gets into your database or your storage, they should find nothing they can read.

**Delete what you don't need anymore.** Old accounts, expired orders, stuff from years ago that nobody uses. Set it up so it gets deleted automatically, not "someday."

**Don't store tokens or API keys** unless you really, really, really, really have to. If you do, encrypt them hardcore or put them in a secrets manager. And either way, keep your encryption keys away from the data they protect.

The less data you keep, the less you have to worry about when you get hacked.
