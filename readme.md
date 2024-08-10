# fast-git-prompt

A fast git prompt for zsh and bash.

> This is a work in progress.
> More features will be added in the future.

## Installation

```bash
cargo install fast-git-prompt
```

Make sure you have `$HOME/.cargo/bin` in your `$PATH`.

## Usage

Include `fast-git-prompt` in your `.zshrc` or `.bashrc` file as part of your prompt.

## Configuration

Create a file called `config.json` in your `$XDG_CONFIG_HOME/fast-git-prompt` or `$HOME/.config/fast-git-prompt` directory.
The configuration of the prompt is fully modular and customizable.

### Example

````json
{
  "version-do-not-modify": "0.1.0",
  "schema": "$XDG_CONFIG_HOME/fast-git-prompt/schema.json",
  "baseColor": "white",
  "prompt": [
    // Your prompt parts go here
  ]
}

### Prompt Parts

#### Branch Name

The branch name is the name of the current branch.

```json
{
  "type": "branchName",
  "color": "white" // Optional
}
````

#### Origin Icon

The origin icon is the icon of the current branch's remote.

```json
{
  "type": "originIcon",
  "icons": {
    "github.com": {
      "icon": "",
      "color": "white"
    },
    "gitlab.com": {
      "icon": "",
      "color": "brightRed"
    }
  },
  "defaultIcon": {
    "icon": "",
    "color": "orange"
  }
}
```

#### Branch Status

The branch status is the status of the current branch.

```json
{
  "type": "BranchStatus",
  "dirty": {
    "color": "red",
    "icon": "✗"
  },
  "clean": {
    "color": "green",
    "icon": "󰸞"
  },
  "deleted": {
    "color": "red",
    "icon": ""
  },
  "changed": {
    "color": "yellow",
    "icon": ""
  },
  "new": {
    "color": "yellow",
    "icon": ""
  }
}
```

#### Branch Sync

The branch sync is the sync status of the current branch.

```json
{
  "type": "BranchSync",
  "ahead": {
    "icon": "↑"
  },
  "behind": {
    "icon": "↓"
  }
}
```

### Colors

You can currently only use ansi colors. Which will use by your terminal emulator.

- black
- red
- green
- yellow
- blue
- magenta
- cyan
- white
- brightBlack
- brightRed
- brightGreen
- brightYellow
- brightBlue
- brightMagenta
- brightCyan
- brightWhite
