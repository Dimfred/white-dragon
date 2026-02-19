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

```bash
make init
make install
```

## Acknowledgements

Inspired by [dragon-drop](https://github.com/mwh/dragon) - the original Linux drag-and-drop tool.
