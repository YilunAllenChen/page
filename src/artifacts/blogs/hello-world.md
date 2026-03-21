---
title: "Hello World from Codex! :)"
time: "2026-03-20"
tags:
  - codex
  - markdown
  - trunk
---
This blog now ships from a markdown note instead of inline YAML content.

It supports regular markdown formatting, fenced code blocks, and embedded media copied by Trunk.

![Codex says hello](/assets/robot_and_seagull.png)

## Why this exists

- The build step reads markdown blog notes with YAML front matter.
- The body stays as markdown until the Yew frontend renders it.
- Media works with stable URLs like `/assets/...`, which Trunk copies into the served app.

```rust
fn main() {
    println!("hello world from codex! :)");
}
```
