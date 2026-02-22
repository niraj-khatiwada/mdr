# Medium Document

This is a medium-sized markdown document for benchmarking rendering performance.

## Introduction

Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris.

## Code Examples

```rust
fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        n => fibonacci(n - 1) + fibonacci(n - 2),
    }
}
```

```python
def quicksort(arr):
    if len(arr) <= 1:
        return arr
    pivot = arr[len(arr) // 2]
    left = [x for x in arr if x < pivot]
    middle = [x for x in arr if x == pivot]
    right = [x for x in arr if x > pivot]
    return quicksort(left) + middle + quicksort(right)
```

## Table

| Feature | egui | WebView | TUI |
|---------|------|---------|-----|
| Native rendering | Yes | No | No |
| CSS support | No | Yes | No |
| Terminal support | No | No | Yes |
| Image support | Yes | Yes | Partial |
| Mermaid diagrams | No | Yes | No |

## Lists

1. First ordered item
2. Second ordered item
   - Nested unordered
   - Another nested
3. Third ordered item

### Task list

- [x] Implement egui backend
- [x] Implement webview backend
- [ ] Implement TUI backend
- [ ] Add Mermaid support

## Blockquote

> This is a blockquote with **formatted** text.
>
> It spans multiple paragraphs.

## Links and Images

Visit [Rust](https://www.rust-lang.org/) for more information.

---

## Footnotes

Here is a footnote reference[^1].

[^1]: This is the footnote content.

## Final Section

This document tests various markdown features to benchmark rendering speed across different backends. The goal is to have enough content to be meaningful without being so large that it becomes unwieldy.

End of medium document.
