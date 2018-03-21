# zsh-yarn-autocompletions

![](./screenshot.gif)

## Installation

### Download

Please go to the [GitHub releases](https://github.com/g-plane/zsh-yarn-autocompletions/releases)
page and download the latest binary files.

Note that you should choose correct file according to your OS.

### Add as a Zsh plugin

Please run:

```shell
$ mkdir $ZSH_CUSTOM/plugins/yarn-autocompletions
```

Unzip the compressed file you downloaded before
and put files to the directory you created before.

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
