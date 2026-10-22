---
name: write-blog-post
description: The voice and the shape of a post of nunomaduro.com. Use when you write a new post in `content/posts/`, when you edit the text of a post, and when you review a draft of a post.
---

# Write a blog post

## 1. The voice

Write as one developer who talks to one friend at a desk. The reader is a developer, and the reader has no time to lose.

Write each word in lowercase. Keep the uppercase only in an item of code, a command, a path, a version and a name of a product that the code shows in uppercase, such as `PHP` or `README.md`.

Do not put a period at the end of a paragraph, of a line or of a list item. Put a period only between two sentences in one paragraph. Keep a question mark and an exclamation mark only when the sentence needs it.

Write short sentences. Let one short sentence stand alone as a paragraph when it carries the idea.

Use contractions such as `it's`, `don't` and `i've`. Use `i` for the author and `you` for the reader.

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
it's raining today. not the dramatic kind, the slow grey kind that makes you look out the window and decide the run can wait until tomorrow

so here's the plan: coffee, a hoodie, laptop on the couch. if the sun shows up after lunch i'll go outside, if it doesn't i won't pretend to be upset about it
```

## 4. The file

Read `infrastructure/src/repositories/markdown_post_repository.rs` for the front matter that a post needs. Give the file a slug of lowercase words with a hyphen between them. Run `npm run og` to make the image of the post.
