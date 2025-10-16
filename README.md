# MyDiary CLI

A simple, fast, and minimalist command-line diary, written in Rust.

`MyDiary` is a tool for people who want to take notes without taking their hands off the keyboard. All entries are stored as simple Markdown files in a single folder, making them easy to sync, back up, and search with your favorite tools.

---

## Features

* **Blazing Fast Entry Creation**: Add a new entry with a single command, without launching any heavy applications.
* **Editor Integration**: Browse and edit all your notes in your favorite terminal editor (Neovim, Vim, Nano, etc.).
* **Smart Dates**: Quickly jump to specific entries using natural language (`yesterday`, `last monday`, `2 weeks ago`).
* **Simple Markdown Files**: Your data belongs to you. No complex formats or databases—just clean, readable `.md` files.

---

## Installation

#### 1. Prerequisites

Make sure you have the **Rust** toolchain and `cargo` installed. You can find instructions at [rust-lang.org](https://www.rust-lang.org/tools/install).

#### 2. Compilation

Clone this repository and build the project in release mode:
```sh
# From the project's root directory
cargo build --release
```

#### 3. Installation

Copy the compiled binary to a directory that is in your system's `$PATH`. The recommended location is `~/.local/bin/`.

```sh
# Ensure the directory exists
mkdir -p ~/.local/bin

# Copy the program
cp ./target/release/mydiary ~/.local/bin/
```

You can now use the `mydiary` command from anywhere in your terminal.

## Usage
`mydiary` has three main modes of operation.

#### 1. Adding a New Entry
Use the `add` command, followed by your note. You don't need to use quotes.

```sh
mydiary add This is my first entry. It works great!
```
The program will automatically add a timestamped entry to a file named with today's date.

#### 2. Browsing All Entries
Run the program without any arguments to open the notes folder in your default editor:

```sh
mydiary
```
You can now freely navigate between files, search them, and edit them.

#### 3. Opening a Specific Entry
Use the `view` command to jump directly to a note from a specific day.

```sh
# Open today's entry
mydiary view

# Open yesterday's entry
mydiary view yesterday

# Open last Monday's entry
mydiary view last monday

# Open the entry from two weeks ago
mydiary view 2 weeks ago

# Open an entry from a specific date
mydiary view 2025-01-01
```

## Configuration
`mydiary` uses the `$EDITOR` environment variable to open files. Make sure it is set to your favorite editor.

Add the following line to your shell's configuration file (e.g., `~/.bashrc` or `~/.zshrc`):

```sh
export EDITOR="nvim"
```
Apply the changes by restarting your terminal or by running `source ~/.bashrc`.
