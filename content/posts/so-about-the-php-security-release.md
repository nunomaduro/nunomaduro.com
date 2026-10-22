---
title: so, about the php security release
description: what i did with my own apps when php2shell dropped, the people who shipped the fix, and why you should build like you're already going to get hacked
published_at: October 22, 2026
---

the details about these two cves are everywhere, so i won't tell you about them again. this is about my thoughts on all of it, and how the day went for me

## my morning

it was 8am when i woke up to a bazillion messages, literally everywhere. that scared me

so i took down every website i had. some went offline, some got a static page instead. then i deleted a bunch of side projects that were lingering around for no reason. if a project isn't online, nobody can hack it

everything i run is on laravel cloud, so once the team patched the platform, i put it all back online. props to the laravel cloud team

## the people you don't see

a new php version isn't a `git tag`. there are build pipelines, there are packages for every platform, there are mirrors that spread the downloads around the world. and yesterday the whole world wanted all of it at the same time

all of that sits on a very small group of people. some of them aren't even paid for it

we also saw big governments and big corporations tweet about it, complain a bit, and donate exactly zero dollars to the cause. i find that hilarious

this needs to be fixed. or maybe that's just how open source is. i don't even know at this point

## the embargo

this part of the story still confuses me

the fix had an embargo until 14:00 utc. chainguard shipped fixed builds before that, and [dan lorenc explained why](https://www.linkedin.com/posts/danlorenc_i-wanted-to-share-what-happened-with-the-share-7518756991520120832-QX2_/): they saw reports of exploits in the wild and wanted to protect people. the php foundation disagreed

but i don't buy the "protect people" part. a fixed build shows exactly what got fixed, and what got fixed tells you exactly where the hole is. ship that early and you're handing out weapons. not only to hackers, but to anyone with an ai model and a free afternoon

so yeah, this whole story needs to be clarified

## what you should do

update php today: `8.5.12`, `8.4.27`, `8.3.36` or `8.2.35`. if you're on something older than `8.2`, there's no fix for you, so today is the day you upgrade

then the bigger thing. it's 2026. build like you're already going to get hacked, because sooner or later you probably will

**collect less.** only keep the customer data you actually need. anything you never store can't leak

**encrypt everything you keep.** not only passwords and card numbers: names, emails, addresses, photos, uploads, all of it. if someone gets into your database or your storage, they should find nothing they can read

**delete what you don't need anymore.** old accounts, expired orders, stuff from years ago that nobody uses. set it up so it gets deleted automatically, not "someday"

**don't store tokens or api keys** unless you really really really really have to. if you do, encrypt them hardcore or put them in a secrets manager. and either way, keep your encryption keys away from the data they protect

the less data you keep, the less you have to worry about when you get hacked
