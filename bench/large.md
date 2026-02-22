# Large Benchmark Document

This document is designed to stress-test markdown rendering performance.

## Chapter 1: Text Content

Lorem ipsum dolor sit amet, consectetur adipiscing elit. Vivamus lacinia odio vitae vestibulum vestibulum. Cras venenatis euismod malesuada. Nulla facilisi. Etiam non diam sed augue pharetra dignissim. Praesent eu massa vel diam laoreet ultrices. Integer posuere erat a ante venenatis dapibus.

Sed ut perspiciatis unde omnis iste natus error sit voluptatem accusantium doloremque laudantium, totam rem aperiam, eaque ipsa quae ab illo inventore veritatis et quasi architecto beatae vitae dicta sunt explicabo. Nemo enim ipsam voluptatem quia voluptas sit aspernatur aut odit aut fugit, sed quia consequuntur magni dolores eos qui ratione voluptatem sequi nesciunt.

### Subsection 1.1

At vero eos et accusamus et iusto odio dignissimos ducimus qui blanditiis praesentium voluptatum deleniti atque corrupti quos dolores et quas molestias excepturi sint occaecati cupiditate non provident, similique sunt in culpa qui officia deserunt mollitia animi, id est laborum et dolorum fuga.

### Subsection 1.2

Ut enim ad minima veniam, quis nostrum exercitationem ullam corporis suscipit laboriosam, nisi ut aliquid ex ea commodi consequatur? Quis autem vel eum iure reprehenderit qui in ea voluptate velit esse quam nihil molestiae consequatur, vel illum qui dolorem eum fugiat quo voluptas nulla pariatur?

## Chapter 2: Code Blocks

```rust
use std::collections::HashMap;

struct Server {
    routes: HashMap<String, Box<dyn Fn(&Request) -> Response>>,
    middleware: Vec<Box<dyn Fn(&mut Request, &mut Response)>>,
}

impl Server {
    fn new() -> Self {
        Server {
            routes: HashMap::new(),
            middleware: Vec::new(),
        }
    }

    fn get(&mut self, path: &str, handler: impl Fn(&Request) -> Response + 'static) {
        self.routes.insert(path.to_string(), Box::new(handler));
    }

    fn run(&self, addr: &str) -> Result<(), Box<dyn std::error::Error>> {
        println!("Server listening on {}", addr);
        Ok(())
    }
}
```

```python
import asyncio
from dataclasses import dataclass
from typing import List, Optional

@dataclass
class Task:
    id: int
    title: str
    completed: bool = False
    subtasks: Optional[List['Task']] = None

class TaskManager:
    def __init__(self):
        self.tasks: List[Task] = []
        self._next_id = 1

    def add_task(self, title: str) -> Task:
        task = Task(id=self._next_id, title=title)
        self._next_id += 1
        self.tasks.append(task)
        return task

    async def process_all(self):
        await asyncio.gather(*[self._process(t) for t in self.tasks])

    async def _process(self, task: Task):
        await asyncio.sleep(0.1)
        task.completed = True
```

```javascript
class EventEmitter {
  constructor() {
    this.listeners = new Map();
  }

  on(event, callback) {
    if (!this.listeners.has(event)) {
      this.listeners.set(event, []);
    }
    this.listeners.get(event).push(callback);
    return () => this.off(event, callback);
  }

  off(event, callback) {
    const handlers = this.listeners.get(event);
    if (handlers) {
      this.listeners.set(event, handlers.filter(h => h !== callback));
    }
  }

  emit(event, ...args) {
    const handlers = this.listeners.get(event) || [];
    handlers.forEach(handler => handler(...args));
  }
}
```

## Chapter 3: Tables

| Method | Time Complexity | Space Complexity | Stable | In-place |
|--------|----------------|------------------|--------|----------|
| Bubble Sort | O(n²) | O(1) | Yes | Yes |
| Selection Sort | O(n²) | O(1) | No | Yes |
| Insertion Sort | O(n²) | O(1) | Yes | Yes |
| Merge Sort | O(n log n) | O(n) | Yes | No |
| Quick Sort | O(n log n) | O(log n) | No | Yes |
| Heap Sort | O(n log n) | O(1) | No | Yes |
| Radix Sort | O(nk) | O(n+k) | Yes | No |
| Tim Sort | O(n log n) | O(n) | Yes | No |

| Feature | Description | Status | Priority |
|---------|-------------|--------|----------|
| File watching | Auto-reload on save | Done | High |
| Dark mode | OS theme detection | Done | High |
| Mermaid | Diagram rendering | Planned | Medium |
| Search | In-document search | Planned | Low |
| TOC | Table of contents sidebar | Planned | Medium |
| Export | PDF/HTML export | Planned | Low |

## Chapter 4: Lists and Nesting

1. Architecture
   1. Frontend
      - React components
      - State management
      - Routing
   2. Backend
      - REST API
      - GraphQL API
      - WebSocket handlers
   3. Infrastructure
      - Docker containers
      - Kubernetes orchestration
      - CI/CD pipelines
2. Development
   1. Setup
      - Clone repository
      - Install dependencies
      - Configure environment
   2. Testing
      - Unit tests
      - Integration tests
      - End-to-end tests
   3. Deployment
      - Staging
      - Production
      - Rollback procedures

### Task Lists

- [x] Project initialization
- [x] Core markdown parsing
- [x] File watching with debounce
- [x] egui backend
- [x] WebView backend
- [ ] TUI backend
- [ ] Mermaid diagram support
- [ ] Syntax highlighting themes
- [ ] Custom CSS support
- [ ] Plugin system

## Chapter 5: Blockquotes and Formatting

> **Important**: This is a critical note about the system architecture.
>
> The rendering pipeline processes markdown in three stages:
>
> 1. **Parsing** — Convert raw markdown to AST
> 2. **Transformation** — Apply extensions and plugins
> 3. **Rendering** — Output to the target backend
>
> > **Nested quote**: Each backend implements its own rendering strategy.
> > The egui backend uses immediate mode GUI, while WebView uses HTML/CSS.

Text with various formatting: **bold**, *italic*, ***bold italic***, ~~strikethrough~~, `inline code`, and [links](https://example.com).

## Chapter 6: Horizontal Rules and Separators

---

Content between rules.

---

More content.

---

## Chapter 7: Footnotes

The Rust programming language[^rust] provides memory safety without garbage collection[^gc]. Its ownership system[^ownership] ensures that references are always valid.

[^rust]: Rust was originally designed by Graydon Hoare at Mozilla Research.
[^gc]: Instead of a garbage collector, Rust uses a system of ownership with a set of rules that the compiler checks at compile time.
[^ownership]: The ownership rules are: each value has an owner, there can only be one owner at a time, and when the owner goes out of scope the value is dropped.

## Chapter 8: Mixed Content

Here is a paragraph with **bold text**, followed by a code block:

```toml
[package]
name = "mdr"
version = "0.1.0"
edition = "2021"

[dependencies]
comrak = "0.38"
clap = { version = "4", features = ["derive"] }
```

Followed by a table:

| Key | Value | Type |
|-----|-------|------|
| name | mdr | String |
| version | 0.1.0 | SemVer |
| edition | 2021 | Year |

And a list:

- Item with `code`
- Item with **bold**
- Item with *italic*
- Item with [link](https://example.com)
- Item with ~~strikethrough~~

End of large benchmark document.
