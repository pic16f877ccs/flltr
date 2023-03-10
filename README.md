## flltr

A small text formatting tool.

## Description
Formats text by longest line, by inserting characters at the end of the string. You can insert characters at the beginning or end of the text.

#### Usage:

$ paste <(flltr file) <(flltr file)

$ flltr -s' ┌' -c -n'22' -f'─' -e'┐'; flltr -s' │ ' -e' │' <(head -n7 <(LC_ALL="en_US.UTF-8" cal)); flltr -s' └' -c -n'22' -f'─' -e'┘'

## License
GNU General Public License v3.0
