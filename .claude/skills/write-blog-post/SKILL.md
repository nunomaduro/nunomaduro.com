---
name: write-blog-post
description: The voice and the shape of a post of nunomaduro.com. Use when you write a new post in `content/posts/`, when you edit the text of a post, and when you review a draft of a post.
---

# Write a blog post

## 1. The voice

Write as one developer who talks to one friend at a desk. The reader is a developer, and the reader has no time to lose.

Write with correct grammar and normal capitalization. Start each sentence and each heading with an uppercase letter, write `I` in uppercase, and write each name of a product in its official form, such as `PHP`, `Laravel Cloud` or `GitHub`. Write each heading in sentence case.

Put a period at the end of each sentence, the last sentence of a paragraph included. Keep a question mark and an exclamation mark only when the sentence needs it.

Write short sentences. Let one short sentence stand alone as a paragraph when it carries the idea.

Use contractions such as `it's`, `don't` and `I've`. Use `I` for the author and `you` for the reader.

Say the opinion directly. Write "this is better" and not "this can arguably be seen as an improvement".

## 2. The content

Give the point in the first two lines. Do not write an introduction about the history of the subject, and do not write "in this post, we will".

Show the code, the command or the output before you explain it. One example is worth one paragraph.

Cut each sentence that the reader can remove and still understand the post. Cut each filler word, such as `basically`, `simply`, `just`, `really` and `actually`, unless the sentence sounds wrong without it.

Do not use marketing words, such as `powerful`, `seamless`, `blazing fast`, `game changer`, `elevate` or `unlock`. Do not use an em dash.

Do not write a conclusion that says the post again. Stop when the point is done. A last line that tells the reader what to try is enough.

Put one dry joke in one post, at the most. Do not explain it.

## 3. An example

Bad:

```markdown
Today, the weather has been quite rainy. It is important to consider how weather can affect our productivity and plans.

In conclusion, a rainy day can be a great opportunity to stay inside and focus.
```

Good:

```markdown
It's raining today. Not the dramatic kind, the slow grey kind that makes you look out the window and decide the run can wait until tomorrow.

So here's the plan: coffee, a hoodie, laptop on the couch. If the sun shows up after lunch, I'll go outside. If it doesn't, I won't pretend to be upset about it.
```

## 4. The file

Read `infrastructure/src/repositories/markdown_post_repository.rs` for the front matter that a post needs. Give the file a slug of lowercase words with a hyphen between them. Run `npm run og` to make the image of the post.
