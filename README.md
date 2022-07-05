# uc-info

A command line utility for looking up and inspecting unicode characters.

## Usage

```
Usage:
  uc-info [OPTIONS] [ARGS ...]

Positional arguments:
  args                  Arguments to the selected operation

Optional arguments:
  -h,--help             Show this help message and exit
  -d,--decode           Decode codepoints (default)
  -t,--transcribe       Convert codepoints to characters
  -i,--identify         Identify characters
  -s,--search           Search for a character by description
  -h,--highlight        Highlight non-ascii characters
  -f,--file FILE        Specify file
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

Identify characters:

```
$ uc-info -i '“naïve”'
“	0x201c	8220	LEFT DOUBLE QUOTATION MARK
n	0x6e	110	LATIN SMALL LETTER N
a	0x61	97	LATIN SMALL LETTER A
ï	0xef	239	LATIN SMALL LETTER I WITH DIAERESIS
v	0x76	118	LATIN SMALL LETTER V
e	0x65	101	LATIN SMALL LETTER E
”	0x201d	8221	RIGHT DOUBLE QUOTATION MARK
```

Highlight non-ascii characters:

<pre><code>
$ uc-info -h '“naïve”'
<mark style="background-color: #ff00ff;">“</mark>na<mark style="background-color: #ff00ff;">ï</mark>ve<mark style="background-color: #ff00ff;">”</mark>
</code></pre>

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
      0     7f  basiclatin                              	Basic Latin
     80     ff  latinsupplement                         	Latin-1 Supplement
    100    17f  latinextendeda                          	Latin Extended-A
    180    24f  latinextendedb                          	Latin Extended-B
    250    2af  ipaextensions                           	IPA Extensions
    2b0    2ff  spacingmodifierletters                  	Spacing Modifier Letters
    300    36f  combiningdiacriticalmarks               	Combining Diacritical Marks
    370    3ff  greekandcoptic                          	Greek and Coptic
    400    4ff  cyrillic                                	Cyrillic
    500    52f  cyrillicsupplement                      	Cyrillic Supplement
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
