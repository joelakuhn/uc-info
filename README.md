# uc-info

A command line utility for looking up and inspecting unicode characters.

## Usage

```
Usage:
  target/debug/uc-info [OPTIONS] [ARGS ...]

Positional arguments:
  args                  Codepoint to describe. Prefix with x/o for hex/octal.

Optional arguments:
  -h,--help             Show this help message and exit
  -c,--decode           Decode codepoints (default)
  -t,--transcribe       Convert codepoints to characters
  -d,--describe         Describe characters
  -s,--search           Search for a character by description
  -l,--list-blocks      List known blocks
  -b,--block BLOCK      Specify a named block
  --ascii               Consider only the ASCII block
  --emoji               Consider only the emoji block
```

## Examples

Describe a codepoint:

```
$ uc-info 0x2584
▄	0x2584	9604	LOWER HALF BLOCK
```

Transcribe codepoints to a string:

```
$ uc-info -t x68 x65 x6c x6c x6f
hello
```

Describe characters:

```
$ uc-info -d '“naïve”'
“	0x201c	8220	LEFT DOUBLE QUOTATION MARK
n	0x6e	110	LATIN SMALL LETTER N
a	0x61	97	LATIN SMALL LETTER A
ï	0xef	239	LATIN SMALL LETTER I WITH DIAERESIS
v	0x76	118	LATIN SMALL LETTER V
e	0x65	101	LATIN SMALL LETTER E
”	0x201d	8221	RIGHT DOUBLE QUOTATION MARK
```

Search only emoji by description:

```
$ uc-info --emoji -s tomato frog
🍅	0x1f345	127813	TOMATO
🐸	0x1f438	128056	FROG FACE
```

Search entire library by description:

```
$ uc-info -s "quotation mark"
"	0x22	34	QUOTATION MARK
«	0xab	171	LEFT-POINTING DOUBLE ANGLE QUOTATION MARK
»	0xbb	187	RIGHT-POINTING DOUBLE ANGLE QUOTATION MARK
‘	0x2018	8216	LEFT SINGLE QUOTATION MARK
’	0x2019	8217	RIGHT SINGLE QUOTATION MARK
‚	0x201a	8218	SINGLE LOW-9 QUOTATION MARK
‛	0x201b	8219	SINGLE HIGH-REVERSED-9 QUOTATION MARK
“	0x201c	8220	LEFT DOUBLE QUOTATION MARK
”	0x201d	8221	RIGHT DOUBLE QUOTATION MARK
...
```

List named blocks:

```
target/debug/uc-info -l
basiclatin      Basic Latin
latinsupplement Latin-1 Supplement
latinextendeda  Latin Extended-A
latinextendedb  Latin Extended-B
ipaextensions   IPA Extensions
spacingmodifierletters  Spacing Modifier Letters
combiningdiacriticalmarks       Combining Diacritical Marks
greekandcoptic  Greek and Coptic
cyrillic        Cyrillic
cyrillicsupplement      Cyrillic Supplement
...
```

Search named block:

```
target/debug/uc-info -b mathematicaloperators -s integral
∫       0x222b  8747    INTEGRAL
∬       0x222c  8748    DOUBLE INTEGRAL
∭       0x222d  8749    TRIPLE INTEGRAL
∮       0x222e  8750    CONTOUR INTEGRAL
∯       0x222f  8751    SURFACE INTEGRAL
∰       0x2230  8752    VOLUME INTEGRAL
∱       0x2231  8753    CLOCKWISE INTEGRAL
∲       0x2232  8754    CLOCKWISE CONTOUR INTEGRAL
∳       0x2233  8755    ANTICLOCKWISE CONTOUR INTEGRAL
```
