# zsh-yarn-autocompletions

![](./screenshot.gif)

[![Travis](https://img.shields.io/travis/g-plane/zsh-yarn-autocompletions.svg?style=flat-square)](https://travis-ci.org/g-plane/zsh-yarn-autocompletions/)
[![license](https://img.shields.io/github/license/g-plane/zsh-yarn-autocompletions.svg?style=flat-square)](https://github.com/g-plane/zsh-yarn-autocompletions/blob/master/LICENSE)
[![Github Releases](https://img.shields.io/github/downloads/g-plane/zsh-yarn-autocompletions/latest/total.svg?style=flat-square)](https://github.com/g-plane/zsh-yarn-autocompletions/releases)

## Supported yarn commands

- yarn run
- yarn remove
- yarn add

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

For 1.x users, please run:

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

### With a plugin manager

#### [zplug](https://github.com/zplug/zplug)

Update your `.zshrc` file with the following line:

```
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

## Contribution

Any contributions are welcome.

## License

MIT License

Copyright (c) 2018-present Pig Fang
