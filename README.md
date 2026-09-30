# ai-mention-rs

Dependency-free Rust crate for detecting brand mentions in AI-generated
text — case-insensitive matching, token-overlap scoring, and
sentence-level extraction. Built for teams that need to track brand
mentions in AI answers from ChatGPT, Perplexity and Gemini.

A companion to the [AI answer visibility checker at seeninaianswers.com](https://seeninaianswers.com/),
which monitors the same signals across many prompts and engines.

```rust
let s = ai_mention_rs::first_mention_sentence(answer, "Acme Analytics");
```

MIT OR Apache-2.0
