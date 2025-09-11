# vela-cli

The main control tool for the Vela dotfiles.

<details><summary id="dependencies">External dependencies</summary>

-   [`libnotfy`](https://gitlab.gnome.org/GNOME/libnotify) - sending notifications
-   [`swappy`](https://github.com/jtheoof/swappy) - screenshot editor
-   [`grim`](https://gitlab.freedesktop.org/emersion/grim) - taking screenshots
-   [`dart-sass`](https://github.com/sass/dart-sass) - discord theming
-   [`app2unit`](https://github.com/Vladimir-csp/app2unit) - launching apps
-   [`wl-clipboard`](https://github.com/bugaevc/wl-clipboard) - copying to clipboard
-   [`slurp`](https://github.com/emersion/slurp) - selecting an area
-   [`wl-screenrec`](https://github.com/russelltg/wl-screenrec) - screen recording (default)
-   [`wf-recorder`](https://github.com/ammen99/wf-recorder) - screen recording (for NVIDIA GPUs)
-   `glib2` - closing notifications
-   `libpulse` - getting audio device
-   [`cliphist`](https://github.com/sentriz/cliphist) - clipboard history
-   [`fuzzel`](https://codeberg.org/dnkl/fuzzel) - clipboard history/emoji picker

</details>

## Installation

### Package manager (recommended)

If packaged, the CLI is available as `vela-cli-git`. To install it you can use
an AUR helper like [`yay`](https://github.com/Jguer/yay), or manually download the
PKGBUILD and run `makepkg -si`.

e.g. using yay

```sh
yay -S vela-cli-git
```

### Manual installation

Install all [dependencies](#dependencies), then install
[`python-build`](https://github.com/pypa/build),
[`python-installer`](https://github.com/pypa/installer),
[`python-hatch`](https://github.com/pypa/hatch) and
[`python-hatch-vcs`](https://github.com/ofek/hatch-vcs).

e.g. via an AUR helper (yay)

```sh
yay -S libnotify swappy grim dart-sass app2unit wl-clipboard slurp wl-screenrec wf-recorder glib2 libpulse cliphist fuzzel python-build python-installer python-hatch python-hatch-vcs
```

Now, clone the repo, `cd` into it, and build the Rust CLI. Then, to install completions:

- Fish: copy `completions/vela.fish` to `/usr/share/fish/vendor_completions.d/vela.fish`.
- Zsh: copy `completions/_vela` to a directory in your `$fpath` (e.g., `/usr/share/zsh/site-functions/`), then `exec zsh`.

```sh
git clone https://github.com/vela-dots/cli.git
cd cli
cargo build --release --manifest-path ./Cargo.toml
sudo cp completions/vela.fish /usr/share/fish/vendor_completions.d/vela.fish
sudo cp completions/_vela /usr/share/zsh/site-functions/_vela
```

## Usage

All subcommands/options can be explored via the help flag.

```
$ vela -h
usage: vela [-h] [-v] COMMAND ...

Main control tool for the Vela dotfiles

options:
  -h, --help     show this help message and exit
  -v, --version  print the current version

subcommands:
  valid subcommands

  COMMAND        the subcommand to run
    shell        start or message the shell
    toggle       toggle a special workspace
    scheme       manage the color scheme
    screenshot   take a screenshot
    record       start a screen recording
    clipboard    open clipboard history
    emoji        emoji/glyph utilities
    wallpaper    manage the wallpaper
    resizer      window resizer daemon
```

## Configuring

All configuration options are in `~/.config/vela/cli.json`.

<details><summary>Example configuration</summary>

```json
{
    "theme": {
        "enableTerm": true,
        "enableHypr": true,
        "enableDiscord": true,
        "enableSpicetify": true,
        "enableFuzzel": true,
        "enableBtop": true,
        "enableGtk": true,
        "enableQt": true
    },
    "toggles": {
        "communication": {
            "discord": {
                "enable": true,
                "match": [{ "class": "discord" }],
                "command": ["discord"],
                "move": true
            },
            "whatsapp": {
                "enable": true,
                "match": [{ "class": "whatsapp" }],
                "move": true
            }
        },
        "music": {
            "spotify": {
                "enable": true,
                "match": [{ "class": "Spotify" }, { "initialTitle": "Spotify" }, { "initialTitle": "Spotify Free" }],
                "command": ["spicetify", "watch", "-s"],
                "move": true
            },
            "feishin": {
                "enable": true,
                "match": [{ "class": "feishin" }],
                "move": true
            }
        },
        "sysmon": {
            "btop": {
                "enable": true,
                "match": [{ "class": "btop", "title": "btop", "workspace": { "name": "special:sysmon" } }],
                "command": ["foot", "-a", "btop", "-T", "btop", "fish", "-C", "exec btop"]
            }
        },
        "todo": {
            "todoist": {
                "enable": true,
                "match": [{ "class": "Todoist" }],
                "command": ["todoist"],
                "move": true
            }
        }
    }
}
```

</details>
