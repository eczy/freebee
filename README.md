# Freebee

A simple Rust implementation of the classic New York Times [Spelling Bee] game.


## Usage (CLI)
```
USAGE:
    freebee-cli [OPTIONS] --wordlist <WORDLIST>

OPTIONS:
    -h, --help                   Print help information
    -s, --seed <SEED>            Seed for RNG. If none provided, seeds from entropy
    -V, --version                Print version information
    -w, --wordlist <WORDLIST>    Path to wordlist
```

Finding a good wordlist is hard, but a decent option can be found [here].

## In-game Commands:
```
?      - display this message
\rules - display the rules of the game
\grid  - display a matrix letter x length for every word in the puzzle
\2ll   - (two letter list) display a count for the number of words starting with each two letter pair
\solve - print all solutions to the puzzle
\quit  - exit the game
```

For more information on how to use the grid, see [this NYT page].

[Spelling Bee]: https://www.nytimes.com/puzzles/spelling-bee
[this NYT page]: https://www.nytimes.com/2021/07/26/crosswords/spelling-bee-forum-introduction.html
[here]: https://www-personal.umich.edu/~jlawler/wordlist.html
