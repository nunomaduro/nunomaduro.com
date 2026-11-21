---
title: Free Local AI for Coding
description: Run an AI model on your own laptop with Ollama and OpenCode. Free forever, no limits, no internet.
published_at: November 21, 2026
---

You can code with an AI model that runs on your own laptop. It's free forever, it has no limits, and it works with no internet. You need two tools: Ollama runs the model, and OpenCode is the interface you talk to.

<iframe src="https://www.youtube-nocookie.com/embed/xUmxoRF3OsA" title="Run AI Locally (Free, No Limits, No Internet)" loading="lazy" allow="accelerometer; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen></iframe>

OpenCode already ships with a few free models. Type `/models` and search for "free". The catch: they don't stay free forever. A model on your own disk does.

## Install Ollama

Go to [ollama.com](https://ollama.com), click on download, and pick your operating system. On macOS, it gives you one script:

```bash
curl -fsSL https://ollama.com/install.sh | sh
```

It takes a few seconds and asks for your password once.

## Install OpenCode

OpenCode is a coding agent that runs in your terminal. Install it with its script:

```bash
curl -fsSL https://opencode.ai/install | bash
```

Run `opencode --version` to check that it's there.

## Pick a model

Run `ollama` and select the option to launch with OpenCode. It shows you a list of models.

Skip each model that says "cloud". Those cost money. The local ones are at the bottom of the list.

Then check the size. A model has to fit in your memory. Say your laptop has 16 GB: skip each model above that, and pick one such as Qwen 3.5 at 6 GB. The download took about 30 seconds.

## Code with it

Once you pick a model, OpenCode opens with that model selected. If it doesn't open, go to your project and run it yourself:

```bash
opencode
```

Ask it something about your project:

```
What version of Laravel am I using?
```

The first prompt is slower. OpenCode loads its context and its tools on that first call, so the model can read files and edit them later. After that, it gets faster. Not cloud fast, but fine.

Then give it real work:

```
Make the welcome page say hello world.
```

It reads the files, calls its tools, and edits the view. Refresh the page, and there's your "hello world".

## Switch models

Go to [ollama.com/search](https://ollama.com/search) and find a different model, such as Gemma 4 from Google. Pick a size that fits your laptop, copy the name, and run it:

```bash
ollama run gemma4
```

Back in OpenCode, type `/models` and search for it. It's on the list. The first prompt is slow again, then it behaves like Qwen did. Most models do fine on small tasks like this one, so pick by memory first.

## Chat without the tools

The Ollama installer also gives you a desktop app. You chat with each model you downloaded, and it answers much faster than in OpenCode, because there's no context and no tools attached to each prompt.

## Should you use it?

Manage your expectations. If you're used to Claude Code, this isn't the same. Claude is faster and smarter.

A local model is good for two things: a flight or a bad connection, and small tasks you don't want to spend tokens on.

I don't use one. My MacBook Pro M4 Max has 36 GB of memory, which only fits the small models, and the small models don't give me the experience I get from Claude Code. If you have a beast of a laptop, the good local models are within your reach.

Install Ollama, pull a 6 GB model, and ask it to change one page of your app.
