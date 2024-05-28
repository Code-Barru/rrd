# Core functionnality
Display hexadump in the same way xxd would.

## Options
    -b          Generate a binary digit dump.
    -c          Set the number of bytes per lines in the output.
    -C          Puts color in the output.
    -g          Set the number of bytes per group in the output.
    -h          Prints help menu.
    -i          Output in C include file style.
    -l          Limit the number of bytes processed or displayed.
    -o          Output file.
    -ps         Output in postscript continuous hex-dump style.
    -r          Reverse mode. Takes a hexdump and outputs binary.
    -s          Skip a specified number of bytes before processing.


[vim's implementation of xxd](https://github.com/vim/vim/blob/master/src/xxd/xxd.c#L252)
[gfg](https://www.geeksforgeeks.org/xxd-command-in-linux/)

## Format

address: 0000 0000 0000 0000 0000 0000 0000 0000 text

address between `00000000` and `ffffffff`
hex value between `00` and  `ff`
