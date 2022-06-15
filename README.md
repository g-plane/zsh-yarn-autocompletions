# zsh-yarn-autocompletions

![](./screenshot.gif)

[![license](https://img.shields.io/github/license/g-plane/zsh-yarn-autocompletions.svg?style=flat-square)](https://github.com/g-plane/zsh-yarn-autocompletions/blob/master/LICENSE)
[![Github Releases](https://img.shields.io/github/downloads/g-plane/zsh-yarn-autocompletions/latest/total.svg?style=flat-square)](https://github.com/g-plane/zsh-yarn-autocompletions/releases)

*You may also like:*

- [icd](https://github.com/g-plane/icd) - Powerful `cd` command with fuzzy-search tool.

## Supported yarn commands

- yarn run
- yarn remove
- yarn add
- yarn why

You can customize the autocompletion of `yarn add` command.
It's configurable.

## Installation

### Manually

#### Download

Please go to the [GitHub releases](https://github.com/g-plane/zsh-yarn-autocompletions/releases)
page and download the latest binary files.

**Note that you should choose correct file according to your OS.**

#### Add as a Zsh plugin

Unzip the compressed file you downloaded.

For 1.x (or above) users, please run:

```shell
$ ./install.sh $ZSH_CUSTOM/plugins
```

For 0.2 users, please run:

```shell
$ ./install.sh $ZSH_CUSTOM
```

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

### Fig 

[Fig](https://fig.io) adds apps, shortcuts, and autocomplete to your existing terminal.

Install `zsh-yarn-autocompletions` in just one click.

<a href="https://fig.io/plugins/other/zsh-yarn-autocompletions_g-plane" target="_blank"><img src="https://fig.io/badges/install-with-fig.svg" /></a>

### With a plugin manager

#### [zinit](https://github.com/zdharma/zinit)

Update your `.zshrc` file with the following line:

```zsh
zinit ice atload"zpcdreplay" atclone'./zplug.zsh'
zinit light g-plane/zsh-yarn-autocompletions
```

#### [zplug](https://github.com/zplug/zplug)

Update your `.zshrc` file with the following line:

```zsh
zplug "g-plane/zsh-yarn-autocompletions", hook-build:"./zplug.zsh", defer:2
```

## Customize the autocompletion of `yarn add` command

> Since v1.0.0

This plugin support adding your favorite packages name for autocompletion
of `yarn add` and `yarn add --dev` command.

### Adding a configuration file

First, put a new file called `.yarn-autocompletions.yml` in your home directory.

For example:

```shell
$ touch ~/.yarn-autocompletions.yml
```

This file is in YAML format.

### Adding your favorite packages

We assume you want to add `vue` and `react`.
So you can edit that file like this:

```yaml
dependencies:
  - vue
  - react
```

And `dev_dependencies`:

```yaml
dev_dependencies:
  - vue-loader
  - parcel-bundler
```

### Excluding some packages

In fact, this plugin has some builtin packages autocompletion
which can provide out-of-the-box experience, such as `vue` or `react`.

However you don't like those builtin packages.

So you can add it to `exclude` section:

```yaml
exclude:
  - react
```

Now you won't see `react` in autocompletion.

### All sections are optional

Now the whole configuration file looks like:

```yaml
dependencies:
  - vue
  - react
dev_dependencies:
  - vue-loader
  - parcel-bundler
exclude:
  - react
```

Keep in mind that any sections are optional.

## Build from source

Make sure that you've installed [rustup](https://rustup.rs/) first and use rustup to install Rust.

Next, follow the steps below:

```
git clone https://github.com/g-plane/zsh-yarn-autocompletions.git
cd zsh-yarn-autocompletions
cargo build --release
mkdir -p $ZSH_CUSTOM/plugins/yarn-autocompletions
cp ./yarn-autocompletions.plugin.zsh $ZSH_CUSTOM/plugins/yarn-autocompletions/
cp ./target/release/yarn-autocompletions $ZSH_CUSTOM/plugins/yarn-autocompletions/
```

Then, edit your `.zshrc` like this:

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
