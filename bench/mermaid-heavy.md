# Mermaid-Heavy Benchmark

This document contains multiple Mermaid diagrams to test diagram rendering performance.

## Flowchart

```mermaid
flowchart TD
    A[Start] --> B{Is it valid?}
    B -->|Yes| C[Process]
    B -->|No| D[Reject]
    C --> E{More items?}
    E -->|Yes| B
    E -->|No| F[Complete]
    D --> G[Log Error]
    G --> E
```

## Sequence Diagram

```mermaid
sequenceDiagram
    participant U as User
    participant C as Client
    participant S as Server
    participant D as Database

    U->>C: Open file
    C->>C: Parse markdown
    C->>S: Request render
    S->>D: Fetch template
    D-->>S: Template data
    S->>S: Render HTML
    S-->>C: HTML response
    C-->>U: Display result
    U->>C: Edit file
    C->>C: Detect change
    C->>S: Re-render
    S-->>C: Updated HTML
    C-->>U: Live update
```

## Class Diagram

```mermaid
classDiagram
    class Backend {
        <<trait>>
        +run(file_path: PathBuf) Result
    }
    class EguiBackend {
        -markdown: String
        -cache: CommonMarkCache
        -watcher_rx: Receiver
        +run(file_path: PathBuf) Result
        +update(ctx, frame)
    }
    class WebViewBackend {
        +run(file_path: PathBuf) Result
        -build_html(body: str) String
    }
    class FileWatcher {
        +watch_file(path: Path) Receiver
    }
    class MarkdownParser {
        +parse_markdown(content: str) String
    }

    Backend <|.. EguiBackend
    Backend <|.. WebViewBackend
    EguiBackend --> FileWatcher
    WebViewBackend --> FileWatcher
    WebViewBackend --> MarkdownParser
```

## State Diagram

```mermaid
stateDiagram-v2
    [*] --> Idle
    Idle --> Loading: Open file
    Loading --> Rendering: File loaded
    Loading --> Error: Load failed
    Rendering --> Displaying: Render complete
    Error --> Idle: Retry
    Displaying --> Watching: Display ready
    Watching --> Loading: File changed
    Watching --> [*]: Window closed
```

## Gantt Chart

```mermaid
gantt
    title MDR Development Timeline
    dateFormat  YYYY-MM-DD
    section Core
    Markdown parsing       :done, core1, 2024-01-01, 3d
    File watching          :done, core2, after core1, 2d
    CLI interface          :done, core3, after core1, 1d
    section Backends
    egui backend           :done, be1, after core2, 3d
    WebView backend        :done, be2, after core2, 3d
    TUI backend            :active, be3, after be1, 5d
    section Features
    Mermaid support        :feat1, after be2, 4d
    Custom themes          :feat2, after feat1, 3d
    Plugin system          :feat3, after feat2, 5d
```

## Entity Relationship

```mermaid
erDiagram
    DOCUMENT ||--o{ SECTION : contains
    SECTION ||--o{ PARAGRAPH : contains
    SECTION ||--o{ CODE_BLOCK : contains
    SECTION ||--o{ TABLE : contains
    SECTION ||--o{ LIST : contains
    SECTION ||--o{ DIAGRAM : contains
    DIAGRAM {
        string type
        string source
        blob rendered
    }
    CODE_BLOCK {
        string language
        string content
        bool highlighted
    }
    TABLE {
        int columns
        int rows
        string alignment
    }
```

## Pie Chart

```mermaid
pie title Rendering Backend Usage
    "egui" : 45
    "WebView" : 35
    "TUI" : 20
```

End of mermaid-heavy document.
