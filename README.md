# RRD

Display hexadump in the same way xxd would.

## Example

````sh
00000000: 2320 5252 440d 0a0d 0a44 6973 706c 6179  # RRD....Display
00000010: 2068 6578 6164 756d 7020 696e 2074 6865   hexadump in the
00000020: 2073 616d 6520 7761 7920 7878 6420 776f   same way xxd wo
00000030: 756c 642e 0d0a 0d0a 6060 6073 680d 0a55  uld.....```sh..U
00000040: 7361 6765 3a20 7272 642e 6578 6520 5b4f  sage: rrd.exe [O
00000050: 5054 494f 4e53 5d20 3c49 4e50 5554 3e20  PTIONS] <INPUT>
00000060: 5b4f 5554 5055 545d 0d0a 0d0a 4172 6775  [OUTPUT]....Argu
00000070: 6d65 6e74 733a 0d0a 2020 3c49 4e50 5554  ments:..  <INPUT
00000080: 3e20 2020 5468 6520 696e 7075 7420 6669  >   The input fi
...
````

## Usage

```sh
Usage: rrd.exe [OPTIONS] <INPUT> [OUTPUT]

Arguments:
  <INPUT>   The input file to read
  [OUTPUT]

Options:
  -a, --addr-len <ADDR_LEN>  The size of bytes read per lines [default: 0]
  -c, --columns <COLUMNS>    The number of columns [default: 16]
  -g, --group <GROUP>        The number of bytes to group [default: 2]
  -l, --len <LEN>            Stop after N bytes [default: 0]
  -f, --format <FORMAT>      Output format [default: h] [possible values: b, c, h, ps]
  -r, --reverse              Reverse the output
  -s, --skip <SKIP>          Skip N bytes [default: 0]
  -h, --help                 Print help
  -V, --version              Print version
```

> [!WARNING]
> It may have so padding issues.
