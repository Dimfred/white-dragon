# White Dragon

This is 100% AI generated.

A lightweight macOS CLI tool that creates a small draggable window for files. Enables command-line users to drag files into GUI applications without needing a file manager.

## Usage

```bash
# Drag single file (exits after drop)
white-dragon -x /path/to/file.txt

# Drag multiple files
white-dragon -x file1.txt file2.pdf file3.png

# Keep window open after drag
white-dragon /path/to/file.txt
```

## Install

### Homebrew

```bash
brew install Dimfred/tap/white-dragon
```

### From source

```bash
make init
make install
```

## lf Integration

Add this to your `lfrc`:

```bash
cmd white_dragon %{{
    echo "$fx" | tr '\n' '\0' | xargs -0 white-dragon -x
}}

map ds white_dragon
```

## Acknowledgements

Inspired by [dragon-drop](https://github.com/mwh/dragon) - the original Linux drag-and-drop tool.
