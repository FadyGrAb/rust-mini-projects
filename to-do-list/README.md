# Mini Rust Project: CLI To-Do List
A lightweight, terminal-based Task Management application written in Rust. This mini-project demonstrates core systems-programming concepts, clean data modeling, and robust user-input handling without any external crate dependencies.

---

## Features & Functionality

- **Interactive Menu System:** Dynamically renders options from custom structs and enums.
- **Task Lifecycle Management:**
  - **Add:** Create new tasks with user-defined descriptions.
  - **Show:** View all tasks along with their current status (`[ ]` for pending, `[x]` for completed).
  - **Mark Complete:** Flag existing tasks as finished in place.
  - **Remove:** Delete tasks from the list by their display number.
- **Robust Error Handling & Input Parsing:** Safe handling of empty text inputs, invalid index numbers, and out-of-bound selections using pattern matching (`match`, `Option`, `Result`).

---

## Rust Concepts & Language Chapters Highlighted

This project serves as a practical implementation of key chapters from *The Rust Programming Language*:

| Feature / Pattern | Applied Code Location | Rust Book Reference |
| :--- | :--- | :--- |
| **Enums & Structs** | `enum Op`, `struct Task`, `struct MenuItem` | Chapter 5 & Chapter 6 |
| **Associated Functions** | `MenuItem::new()` | Chapter 5.3 (*Method Syntax*) |
| **Ownership & Borrowing** | Slices `&[MenuItem]`, `&[Task]` | Chapter 4 (*Understanding Ownership*) |
| **Heap Allocation** | `Vec<Task>` | Chapter 8.1 (*Vectors*) |
| **Pattern Matching & Control Flow** | `'main_loop: loop`, `match`, guards (`if num > 0`) | Chapter 3.5 & Chapter 6.2 |
| **Safe Indexing & Mutation** | `tasks.get_mut()`, `tasks.remove()` | Chapter 8 & Chapter 4.2 |
| **Standard I/O & Flushing** | `io::stdin()`, `io::stdout().flush()` | Chapter 12 |

---

## Deep Dive: Rust Concepts Explained

### 1. Custom Data Types: Enums and Structs
* **Rust Book Reference:** Chapter 5 & Chapter 6

```rust
enum Op {
    Add,
    Remove,
    Show,
    MarkComplete,
    Quit,
}

struct Task {
    description: String,
    completed: bool,
}
```

- **Enums (`enum Op`):** Represent an algebraic sum type where a value can be one of several discrete variants. Here, `Op` models all application commands.
- **Structs (`struct Task`):** Group related properties into a single record on the heap/stack (`description: String` for dynamic text, `completed: bool` for status).
- **Safety Benefit:** Rust forces exhaustive pattern matching on `enum` types. If you add a new option to `Op`, the code will fail to compile until every `match` statement in the application handles that new variant.

---

### 2. Associated Functions and Constructors
* **Rust Book Reference:** Chapter 5.3

```rust
impl MenuItem {
    fn new(name: &str, operation: Op) -> Self {
        Self {
            name: String::from(name),
            operation,
        }
    }
}
```

- Functions inside an `impl` block that do **not** take `&self` or `&mut self` as their first parameter are **associated functions**.
- `new()` acts as an idiomatic constructor in Rust.
- The parameter `name: &str` accepts a zero-copy string slice reference, which is explicitly copied into an owned `String` buffer on the heap via `String::from(name)`.

---

### 3. Ownership, Borrowing, and Slice References
* **Rust Book Reference:** Chapter 4

```rust
fn show_menu(options: &[MenuItem]) { ... }
fn show_tasks(tasks: &[Task]) { ... }
```

- **Borrowing vs. Moving:** Passing a full `Vec<T>` by value moves ownership into the function, dropping the vector when the function ends.
- **Slices (`&[T]`):** Slices are non-owning views into a contiguous sequence of elements. By borrowing `&[MenuItem]` and `&[Task]`, viewer functions inspect lists without consuming them or cloning memory.
- **Borrowing Rules:** Taking immutable references (`&`) allows multiple read-only views simultaneously without risk of data races.

---

### 4. Dynamic Heap Allocation via Vectors (`Vec<T>`)
* **Rust Book Reference:** Chapter 8.1

```rust
let mut tasks: Vec<Task> = Vec::new();
```

- `Vec<T>` allocates memory dynamically on the heap to store an arbitrary number of items.
- Marking the variable as `mut` gives the program permission to modify vector memory at runtime (`tasks.push()`, `tasks.remove()`).

---

### 5. Control Flow, Labeled Loops, and Pattern Matching
* **Rust Book Reference:** Chapter 3.5 & Chapter 6.2

```rust
'main_loop: loop {
    // ...
    match input_buff.trim().parse::<usize>() {
        Ok(num) if num > 0 => match options.get(num - 1) {
            Some(item) => match item.operation {
                // ...
                Op::Quit => break 'main_loop,
            },
            None => println!("Choice out of bound!"),
        },
        _ => println!("Invalid input!\n"),
    }
}
```

- **Labeled Loops (`'main_loop:`):** Allows deeply nested constructs (like nested `match` statements) to break out of the outer event loop directly when `Op::Quit` is selected.
- **Match Guards (`if num > 0`):** Adds conditional filtering on a `match` arm after pattern matching succeeds.
- **Null Safety:** Rust replaces null pointers with `Option<T>` (`Some`/`None`) and `Result<T, E>` (`Ok`/`Err`), forcing explicit error handling at compile time.

---

### 6. Safe Mutable Indexing (`get_mut`)
* **Rust Book Reference:** Chapter 8 & Chapter 4.2

```rust
match tasks.get_mut(idx - 1) {
    Some(task) => {
        task.completed = true;
    }
    None => println!("Index out of bound!"),
}
```

- Direct indexing (`tasks[i]`) causes an immediate thread panic if index `i` is out of bounds.
- `get_mut()` safely returns an `Option<&mut Task>`. If valid, it returns `Some` with a mutable reference to modify the task in place; if invalid, it yields `None` without crashing.

---

### 7. Standard I/O Buffering and Explicit Flushing
* **Rust Book Reference:** Chapter 12.1

```rust
fn show_prompt(message: &str) {
    print!("{message}: ");
    io::stdout().flush().ok();
}
```

- Standard output in terminal environments is line-buffered by default (output isn't displayed until a newline `\n` is printed).
- Calling `io::stdout().flush()` forces prompt text to display immediately before user input is requested.

---

## How to Compile and Run

1. **Create a new Rust binary package** (if not already inside one):
   ```bash
   cargo new todo_cli
   cd todo_cli
   ```

2. **Replace the contents of `src/main.rs`** with the application code.

3. **Build and run the project** using Cargo:
   ```bash
   cargo run
   ```

---

## Example Session

```text
Choose one of the following options:
	[1] Add
	[2] Remove
	[3] Show
	[4] Mark Complete
	[5] Quit
>> 1
Enter New Task: Learn Rust pattern matching
Task is added.

Choose one of the following options:
	[1] Add
	[2] Remove
	[3] Show
	[4] Mark Complete
	[5] Quit
>> 3
Current tasks:
  1 - Learn Rust pattern matching [ ]

Choose one of the following options:
	[1] Add
	[2] Remove
	[3] Show
	[4] Mark Complete
	[5] Quit
>> 4
Current tasks:
  1 - Learn Rust pattern matching [ ]

Enter task number: 1
Task is marked as Completed.

Choose one of the following options:
	[1] Add
	[2] Remove
	[3] Show
	[4] Mark Complete
	[5] Quit
>> 3
Current tasks:
  1 - Learn Rust pattern matching [x]

Choose one of the following options:
	[1] Add
	[2] Remove
	[3] Show
	[4] Mark Complete
	[5] Quit
>> 5
```