# hexgrep

This tool can grep binary files in text or hexadecimal input.

## how to install
``` bash
cargo install --git https://github.com/umaumax/hexgrep
```

## how to use
``` rust
$ hexgrep --in /bin/ls --text 'https://'
000120b0  f7 63 64 05 01 30 81 f2  30 2a 06 08 2b 06 01 05  |.cd..0..0*..+...|
000120c0  05 07 02 01 16 1e 68 74  74 70 73 3a 2f 2f 77 77  |......https://ww|
000120d0  77 2e 61 70 70 6c 65 2e  63 6f 6d 2f 61 70 70 6c  |w.apple.com/appl|

00025ed7  82 01 00 06 09 2a 86 48  86 f7 63 64 05 01 30 81  |.....*.H..cd..0.|
00025ee7  f2 30 2a 06 08 2b 06 01  05 05 07 02 01 16 1e 68  |.0*..+.........h|
00025ef7  74 74 70 73 3a 2f 2f 77  77 77 2e 61 70 70 6c 65  |ttps://www.apple|
```

### help
``` rust
hexgrep 0.1.0

USAGE:
    hexgrep [OPTIONS] --hex <hex-pattern> --in <input> --text <text-pattern>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --color <color-when>     use markers to highlight the matching strings; WHEN is 'always', 'never', or 'auto'
                                 [default: auto]
    -C <context>                 print NUM bytes of output context [default: 16]
        --hex <hex-pattern>      e.g. --hex 6e616e6f6861 or use --text
    -i, --in <input>             target input binary file
        --text <text-pattern>    e.g. --text rust or use --hex
```
