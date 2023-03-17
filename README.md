## flltr

A small text formatting tool.

## Description

Formats text by longest line, by inserting characters at the end of the string. You can insert characters at the beginning or end of the text.
### Installation

Build and install with Rust package manager.
```console
cargo install flltr
```
Installing the [package](https://github.com/pic16f877ccs/flltr/releases/download/v0.2.1/flltr-0.2.1-x86_64.pkg.tar.zst) using the Arch package manager.
```console
sudo pacman -U ./flltr-0.2.1x86_64.pkg.tar.zst
```

Installing the [package](https://github.com/pic16f877ccs/flltr/releases/download/v0.2.1/flltr_0.2.1_amd64.deb) using the Ubuntu package manager.
```console
sudo apt install ./flltr_0.2.1_amd64.deb
```

### Usage:
```console
paste <(ls | flltr) <(ls -1)
```
![Command paste output.](/images/PasteOutput.png "The command flltr output.")

The ls command
```console
C='red'; c=$(ls); n=$(($(wc -L <<<$c)+2)); flltr -s' ┌' -c -n$n -f'─' -e'┐' -C$C; flltr -s' │ ' -e' │' <<<$c -C$C; flltr -s' └' -c -n$n -f'─' -e'┘' -C$C
```
![Command ls output.](/images/LsOutput.png "The command flltr output.")

## License
GNU General Public License v3.0
