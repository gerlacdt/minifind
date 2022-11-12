## minifind

A learning project which has similar functionality as `find`.

It support various features:

* find files recursively from a given directory
* filter filenames via Perl-style regular expressions, [regex](https://docs.rs/regex/latest/regex/)

### Example usage

``` bash
# after compilation with cargo build, you can run the binary

# run without options to see help
target/debug/minifind

# more detailed help
target/debug/minifind --help

# search recursively in current directory
target/debug/minifind .

# search recursively in current directory for filenames including "git"
target/debug/minifind . --pattern "git"
```


### Development

``` bash
# build project
cargo build

# run tests
cargo test
```

#### More features

Not yet thought of.
