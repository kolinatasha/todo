# Todo CLI

A simple, fast todo list application written in Rust. Persist your tasks to JSON.

## Requirements

- **Rust 1.70+** — [Install from rustup.rs](https://rustup.rs)
- **Cargo** — Comes with Rust

## Setup & Build

### Clone or navigate to the project
```bash
cd todo
```

### Build the project
```bash
cargo build --release
```

The executable will be at `target/release/todo.exe` (Windows) or `target/release/todo` (Linux/Mac).

### Run directly (development)
```bash
cargo run -- [COMMAND] [OPTIONS]
```

## Usage

### Add a task
```bash
cargo run -- add "buy bread"
cargo run -- add "finish homework"
```

### List all tasks
```bash
cargo run -- list
```

Output:
```
[ ] [1] buy bread
[ ] [2] finish homework
```

### Mark a task as done
```bash
cargo run -- done 1
```

Output:
```
[x] [1] buy bread
[ ] [2] finish homework
```

### Remove a task
```bash
cargo run -- rm 2
```

### Clear all completed tasks
```bash
cargo run -- clear-done
```

Removes all tasks marked with `[x]`.

### Specify a custom file location
```bash
cargo run -- --file ./my_todos.json add "task"
```

Default file: `todos.json` (created in current directory)

## Project Structure

```
todo/
├── Cargo.toml              # Dependencies: serde, serde_json, clap
├── README.md               # This file
├── todos.json              # Your tasks (auto-created)
└── src/
    ├── lib.rs              # Library exports (model, store)
    ├── main.rs             # CLI app using the library
    ├── model.rs            # TodoList, Task structs + logic
    └── store.rs            # Load/save functions + tests
```

## Recreate from Scratch

### Step 1: Create a new project
```bash
cargo new todo
cd todo
```

### Step 2: Add dependencies
```bash
cargo add serde --features derive
cargo add serde_json
cargo add clap --features derive
```

### Step 3: Create modules

**src/model.rs** — Define Task and TodoList structs with methods (add, remove, mark_done, etc.)

**src/store.rs** — Implement load() and save() functions to persist todos to JSON

**src/lib.rs** — Export the modules:
```rust
pub mod model;
pub mod store;
```

### Step 4: Build the CLI

**src/main.rs** — Use clap to parse CLI arguments and implement command handlers

### Step 5: Test
```bash
cargo test
```

### Step 6: Build release binary
```bash
cargo build --release
```

## Example Workflow

```bash
# Add tasks
cargo run -- add "Learn Rust"
cargo run -- add "Build a project"
cargo run -- add "Make coffee"

# List them
cargo run -- list

# Mark first as done
cargo run -- done 1

# List again
cargo run -- list

# Clear completed
cargo run -- clear-done

# List final
cargo run -- list
```

## Features

✅ Add, list, mark done, remove tasks  
✅ Persistent storage (JSON)  
✅ Custom file path support  
✅ Error handling (task not found)  
✅ Full test coverage  

## Testing

```bash
cargo test
```

Runs all unit tests for model and store modules.
