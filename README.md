# Axumlings

Greetings and welcome to `axumlings`. This project contains small exercises to get you used to Axum, Tokio, and Rust web development!

In this project, the solutions are hidden from you by default! You have to fix the problems in each `exercises/` file in order to pass to the next one. Only after you successfully write a correct solution will the real solution code become available in the `solutions/` folder for you to compare and see idiomatic Axum code!

## Getting Started

1. Clone the repository:

```bash
git clone https://github.com/YOUR_USERNAME/axumlings.git
cd axumlings
```

2. Run the exercises in Watch mode!

```bash
cargo run watch
```

Alternatively you could run `cargo run` with no arguments, and the CLI will start interactive watch mode.

Axumlings will compile the exercises and wait for you to fix the current one! Whenever you get one passing, its corresponding folder in `solutions/` will populate with the official solution, designed by the `rust-pro` skill so you can compare!

## CLI Options

The CLI acts just like `rustlings`:

**Watch mode (recommended):**

```bash
cargo run watch
```

Alternatively:

```bash
cargo run
```

This runs a continuous loop that checks your current exercise whenever a file is saved.

**Other helpful commands:**

- `cargo run verify` - Verifies all exercises in order.
- `cargo run list` - Lists all exercises and their pending/done statuses.
- `cargo run run [name]` - Re-runs a specific exercise.
- `cargo run hint [name]` - Shows the hint for a specified exercise!

## Using the CLI Watch Loop

When running in interactive mode, these shortcuts are available:

- **h** - Show hint for current exercise
- **n** - Move to next exercise (after solving current)
- **c** - Verify all exercises
- **x** - Reset current exercise
- **q** - Quit

## Adding new Exercises

If you want to contribute, you can map your new exercise inside `info.toml`.
Any new hints you want to offer belong in `info.toml` so they don't spoil users looking around in `exercises/`! If you want to embed solutions to help out, add your solution to the corresponding subfolder inside `.solutions/`. You're done! `axumlings` handles the rest automatically.

## License

MIT
