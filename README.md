# pda2cfg

This project takes in a text description of a [Pushdown Automaton](https://en.wikipedia.org/wiki/Pushdown_automaton) (PDA) and converts it to a [Context-free Grammar](https://en.wikipedia.org/wiki/Context-free_grammar) (CFG).

## Format
Format of the PDA:`Q | Σ | Γ | q_0 | F | ∂`
* Where Q is a finite set of states (x,y)
* Where Σ is a finite set which is called the *input* alphabet (alphanumerics, $)
* Where Γ is a finite set which is called the *stack* alphabet (alphanumerics, $)
* Where q_0 ∈ Q is the start state (x)
* Where F ⊆ Q is the set of accepting states (x,y)
* Where ∂ is a finite subset of `Q × (Σ∪{ϵ}) × Γ × Q × Γ^*`, the *transition relation*
* * `(state, input, symbol, next state, new symbol)`

All sets are comma-seperated.
Whitespace is ignored.

`~` is a stand-in for epsilon transitions and the blank symbol.

## Running
To run, simply use Rust's `cargo run` feature, and provide the name of a file containing the text description of your input PDA as the first argument.

To generate a pda2cfg.exe file, use `cargo build --release` which can then be ran from command-line and provided the PDA argument.

## Contributing
I used [Pest](https://pest.rs/) for the PDA parser, [Serde](https://serde.rs/) for the seralizer, and [SNAFU](https://docs.rs/crate/snafu) for broken error handling.