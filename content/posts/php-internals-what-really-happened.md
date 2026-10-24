---
title: PHP Internals: What Really Happened (w/ Derick Rethans)
description: The full transcript of my interview with Derick Rethans about the PHP security release, from the 2024 audit to the leaks before the embargo.
published_at: October 24, 2026
---

<iframe class="aspect-video w-full rounded-2xl" src="https://www.youtube.com/embed/5X5csEbmHqs" title="PHP Internals: What Really Happened (w/ Derick Rethans)" loading="lazy" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture" allowfullscreen></iframe>

If you prefer the video version of this conversation, it's right above. If you'd rather read, keep scrolling.

I sat down with Derick Rethans to clear up what happened with PHP in the past week. Derick is a member of the PHP Foundation and a contributor to PHP internals, and he knew exactly how it all went down.

There's a lot of misinformation out there, so I wanted this one to be about the facts, not about me. Below is the full transcript, lightly edited so it reads well.

## Is the latest version safe?

**Nuno:** There is some misinformation on Twitter, which is obviously expected. People think that there are still vulnerabilities, even on the latest version of PHP. Can you clarify this for us, please?

**Derick:** Well, it is very likely that there are still vulnerabilities, whether we know which ones they are or not. We can't say, because we're getting so many reports that we need to figure out whether these are credible, whether these are accurate or not. Now, I've seen a few things posted that we know are vulnerabilities, but not on the same scale as what we had with php2shell. So that's what's going on. We just put out an announcement for another vulnerability that we sort of knew existed, but it's not critical in a way that we need to make a new version immediately, because it's such a hard thing to trigger. Like any software, there will be vulnerabilities still.

**Nuno:** Of course. So if I want to be safe against the vulnerabilities that got public, I have to pick the latest version, and then I can feel safe regarding that.

**Derick:** Absolutely. As far as we know for sure now, yes.

## 2024 or August 2026?

**Nuno:** Now, jumping a little bit into the timeline of the events. Some reports say that this vulnerability got reported in 2024. Other reports say that this vulnerability got reported in August this year. I wonder if you could clarify this a little bit.

**Derick:** In 2024, we had an audit done, a funded audit through the STF, funded by the German government. In their report, there was a hint that we had a possible problem in this situation. But neither they nor we thought it could be exploited or anything like that. We did know that something could potentially go wrong under certain circumstances, but we didn't know enough to make any statement of whether it could be exploitable at some point.

In August 2026, through the PHP security ecosystem, people reached out to Hack the Planet, and through that, three different organizations used AI models to try to find bugs in PHP. They found the same or a very similar issue independently, and they could prove that it was exploitable in certain circumstances.

**Nuno:** That was clear. I assume AI was able to accelerate these kinds of findings, and also provide a meaningful report of how to reproduce this. And that's why, in August this year, you acknowledged that there is actually a security vulnerability here.

## What about older versions?

**Nuno:** Some people are using older versions of PHP. Can you confirm that the only solution for this is to update to the supported ones?

**Derick:** The PHP project only makes releases available for the supported versions, 8.2 to 8.5. But we have backported some fixes as well, so there are branches available. In the announcements, I believe PHP 7.4 and up have branches, but you'll have to compile things yourself. And there are also third-party providers that will have backported fixes, even for PHP 5 releases. I think Ondřej Surý has done that, and I'm sure the third-party providers that offer long-term support for PHP have also done so.

**Nuno:** And I assume your recommendation is to update, because it's probably the easier path.

**Derick:** Correct. Yes. But if you're running an older code base that you haven't tested with a newer version of PHP, then that takes more time. You should still absolutely do it, for this specific reason: if there's a security release, you can update much, much quicker if you're already running the latest supported PHP version.

**Nuno:** TL;DR of this whole story, folks: update as soon as possible.

## Why not fix it in 2024?

**Nuno:** What prevented PHP internals from doing a release in 2024, right after you found out about this, with a bug fix?

**Derick:** Well, although we were aware of it, the fix for it is not simple. It has to do with memory, with the memory allocator, and when you run out of memory, things go wrong. Doing a good fix for it is not a trivial thing. It's not like we were not looking at it. We were definitely looking at it for some time, but we could never come up with a good and credible patch that also wouldn't affect extensions and things like that. So that took some time.

And there are so many other things to do. The security team is a small team. It's bigger now, because of the acceleration of AI tools trying to find vulnerabilities and things like that. And because of that, there's just no time. You can't afford to spend that much time on something when you don't know if it's going to be a major issue. A crash, sure, but beyond that, we didn't know. And the PHP Foundation, with so few people, can't prioritize everything at the same time, right?

## From August 2026 to October 2026

**Nuno:** Derick, could you elaborate a little bit on what happened between August and October? How did people learn about this vulnerability, who leaked it, and why did everything go crazy on the internet?

**Derick:** In the last month and a half, we worked on a patch, verified the patch, made sure it's actually exploitable, and things like that. It's like normal software development. And at some point, you need to start talking to partners that build PHP, like Linux distributions, or Docker, or basically anybody else that builds PHP in some way. That includes reaching out to some organizations that deploy a lot of PHP, like WordPress and things like that, right?

**Nuno:** And all of that happens privately?

**Derick:** Yeah, all privately, because it has to. These processes take time. People need to test deployments and need to test whether builds work. You need to come up with press releases and websites. It's not as simple as just publishing a tarball. There's a lot of work around it, especially with a potentially serious problem like this. And that means you need to talk to third parties. And the more parties you tell something, the more chance there is that it might leak, right? It's the same reason why I don't think any conspiracy theory about anybody hiding anything from you will ever work. Even with 200 or 300 people, it gets leaked, right? So anything complicated can potentially get leaked.

## The three leaks

**Derick:** There were three different situations where something came out.

There was a post on Hacker News from a person who didn't know it was under embargo, so he thought, "Well, I can talk about it." He's apologized. It's unfortunate, but I don't think you can blame a person for disclosing information when he didn't know he shouldn't disclose it, right? The information must have come from somewhere else, and we don't know where.

Then later, Calif, the people that made a website with an exploit on it, leaked the video of it, but not the details.

And then there was another organization. They decided on their own to put their builds online, because they are claiming, and I need to be careful with my words here, that they knew there was a vulnerability that was being exploited live. So they decided to publish their builds to their customers only. I think that is really unfortunate. You should never, ever break your embargoes. Also, about the claims they have, I don't think we have seen any substantiated evidence. And I think Nils Adermann had a long post on LinkedIn saying, "Well, we don't know anything about this. And if you think this happened, why didn't you talk to us?" So I think that was a really bad thing for them to do.

And in my opinion, and this is my opinion, not the PHP Foundation's, it's an organization we probably shouldn't include in the private list of companies that we contact if something else happens in the future. That might not be the most politically expedient way of doing things, obviously.

So those were the three specific leaks. The one where it got announced earlier, I think with the leak of the exploit from Calif, is where we decided to push the release itself a day forward.

**Nuno:** Interesting. So the release was scheduled, and in light of all these leaks online and everyone talking about it, you decided to release it earlier.

**Derick:** Yeah. Well, it's not quite like that. If you talk to so many people, you can sort of expect that there's a possibility of this happening. So we were aware this could happen. On our side, it wasn't really a rush to get things done a day earlier, but it was still annoying and unfortunate. I think in the end it worked out. It just meant that the Docker builds showed up a little bit later than we would have hoped.

## Websites that went offline

**Nuno:** We saw a lot of government websites go offline just to protect themselves until they got updated. We saw various well-known websites out there doing the same. What are your thoughts on this, if any?

**Derick:** Well, it's hard to say, right? From the PHP point of view, we did not make that recommendation, but we also don't know what the update process of all these companies and organizations is, right? So it is up to them to make that decision. I think in some cases it's probably wise to do. For the Xdebug websites, I did it myself, even though I knew what was going on, to be on the safe side. For me, that wasn't a problem, because I also knew that once Debian had the new packages, I could type `apt update`, `apt upgrade`, and it literally takes 3 minutes, right?

So as long as you're prepared to upgrade your software quickly, this is not a problem whatsoever. But maybe you don't know where your PHP servers run, or what version they run. Maybe there's a PHP 4 application somewhere, right? It happens. In these situations, if you need to do more work to figure out what you're actually running, it takes more time to figure out what you need to patch. So sometimes it's probably expedient to take down your servers momentarily to be able to patch.

To be fair, I think most organizations were back up really quickly as well. Those are the organizations that take things seriously. In the UK, the National Cyber Security Centre managed to send out warnings to almost every business they had email addresses for. I myself got two, which is kind of funny. And I think that is really useful, because they take seriously that there could be a serious problem.

I think, especially in the UK, the news was a little bit overblown. "AI is finding all the problems," blah, blah, blah. Although in this case they were confirmed, they weren't actually found that way. And the scaremongering around that, I don't like very much. But then again, in the end, if it gets people to talk about cybersecurity, and to be aware that you need to run the latest version of software and be ready to install updates on a whim, I think that is useful. So I don't mind it being overhyped. It's not quite the whole story behind it, but I can see how that works.

## How you can help

**Nuno:** To finish this chat, I want to give you the opportunity to explain to people how they can contribute to make the PHP Foundation, internals, core, or whatever we're going to call all of this, a little bit better. If I'm a business owner, what can I do? If I'm a developer, what can I do?

**Derick:** If you're a developer, talk to the company you work for: your company needs to help the PHP Foundation. In most cases, see whether you have money available, money that you make by using PHP and other open source software, because PHP itself is not going to be the only open source project that you run in your company, right? Most of this software is all done by volunteers.

The PHP Foundation's budget is small compared to any large corporation, or any medium-sized organization, right? It operates on a budget of what, £400,000 a year or something like that? That's almost nothing.

So the best thing you can do is fund the open source projects that you rely on as a business, because that gets more eyes on them, and potentially more people that get paid to work on them. Unfortunately, we don't live in a time anymore where you, as a single developer, can support a project that is used by large corporations and be expected to make sure everything is safe, right? That's no longer realistic.

So I think we need a lot more professionalism in supporting these organizations, in this case the PHP Foundation, but also any other open source project, so the developers have time, or can make time, by being funded to work on these things. And I hope this incident will make sure that the PHP Foundation, as well as Remi's repository and Ondřej for the Debian and RPM packages, have the time to do so. I think that is a good takeaway from this. Make sure you support the people that your company or your organization relies on.
