## flltr

A small text formatting tool.

## Description

Formats text by longest line, by inserting characters at the end of the string. You can insert characters at the beginning or end of the text.
#### Installation
```console
cargo install flltr

```
#### Usage:
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
