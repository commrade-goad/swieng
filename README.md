# SWIENG
a simple template switch engine.

## BUILD

```sh
cargo build --release
```

## USAGE

Usage and How-to can be seen from the `test` folder and try to run the command below.

```sh
./target/release/swieng -i test/color.conf -d test/dict.toml -o new-color.conf
```

| Args                   | Function                           |
|------------------------|------------------------------------|
| `-i` or `--input`      | Input file                         |
| `-o` or `--ouput`      | Output file                        |
| `-d` or `--dictionary` | dictionary file                    |
| `-e` or `--delimiter`  | delimiter to use (default use `%`) |
