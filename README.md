# zsh-yarn-autocompletions

![](./screenshot.gif)

## Installation

You need to compile the Rust code by yourself,
so Rust development is needed.

### Steps to compile

First please make sure you have install Rust and Cargo.

Then you can run:

```bash
$ cd yarn-deps
$ cargo build --release
$ cd ..
$ cd yarn-scripts
$ cargo build --release
```

The binary files are located in `yarn-deps/target/release/yarn-deps`
and `yarn-scripts/target/release/yarn-scripts`.

### Add as a Zsh plugin

Once you compiled the source code to binary,
please run:

```shell
$ mkdir $ZSH_CUSTOM/plugins/yarn-autocompletions
```

Then put the `yarn-autocompletions.plugin.zsh` file
and the binary files to the directory you created before.

Next please edit your `.zshrc` file.
Add `yarn-autocompletions` to `plugins` section.

Like this:

```diff
plugins=(
  # ... your other plugins
+ yarn-autocompletions
)
```

Restart your terminal.

## Contribution

Any contributions are welcome.

## License

MIT License

Copyright (c) 2018-present Pig Fang
