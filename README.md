# uc-info

A command line utility for looking up and inspecting unicode characters.

## Usage

```
Usage:
  uc-info [OPTIONS] [CODEPOINT ...]

Positional arguments:
  codepoint             Codepoint to describe. Prefix with x/o for hex/octal.

Optional arguments:
  -h,--help             Show this help message and exit
  -c,--decode           Decode codepoints (default)
  -t,--transcribe       Convert codepoints to characters
  -d,--describe         Describe characters
  -s,--search           Search for a character by description
  --ascii               Consider only the ASCII block
  --emoji               Consider only the emoji block
```
