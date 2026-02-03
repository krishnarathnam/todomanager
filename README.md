## Todo Manager

This is a simple command-line **Todo Manager** written in Rust. It is designed to help you create, manage, and track your tasks directly from the terminal.

### Features

- **Add todos**: Quickly add new tasks to your list.
- **List todos**: View all your current tasks.
- **Toggle completion**: Mark tasks as done or undo them.
- **Delete todos**: Remove tasks you no longer need.
- **Sort todos**: Show pending tasks first and completed ones last.
- **Reset list**: Clear all todos in one go.
- **Persisted storage**: Keep your tasks saved between runs in `~/.todomanager/todo.json`.

### Functionality and examples

- **Add a todo**
  - **What it does**: Creates a new todo item with a title, timestamp, and unique ID.
  - **Example**:
    ```bash
    todomanager add "Buy groceries"
    ```

- **List todos**
  - **What it does**: Shows all current todos with their ID, title, status (done/pending), and creation time.
  - **Example**:
    ```bash
    todomanager list
    ```

- **Toggle a todo**
  - **What it does**: Marks a todo as done if it is pending, or makes it pending again if it is done.
  - **Example**:
    ```bash
    todomanager toggle 42
    ```
    Here `42` is the ID shown in the list output.

- **Remove a todo**
  - **What it does**: Permanently deletes a todo by its ID.
  - **Example**:
    ```bash
    todomanager rm 42
    ```

- **Sort todos**
  - **What it does**: Reorders the list so that all pending todos appear before completed ones.
  - **Example**:
    ```bash
    todomanager sort
    ```

- **Reset all todos**
  - **What it does**: Deletes **all** todos by clearing the underlying storage file.
  - **Example**:
    ```bash
    todomanager reset
    ```

- **Show help**
  - **What it does**: Prints a table of available commands and their descriptions.
  - **Example**:
    ```bash
    todomanager help
    ```

### Running the project locally

You can install and run this project locally with Cargo:

```bash
cargo install --path .
```

After installation, you can run the installed binary directly from your terminal (usually by using the package name or binary name defined in `Cargo.toml`).

### Requirements

- Rust toolchain (Cargo and `rustc`) installed. You can install it from [`https://rustup.rs`](https://rustup.rs).

### Contributing

Feel free to open issues or submit pull requests to improve the Todo Manager, add new features, or fix bugs.
