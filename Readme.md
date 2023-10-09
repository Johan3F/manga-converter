# Manga converter

This small program takes a `.cbz` files as an input, and returns a pdf formatted with two manga pages per pdf page. It will make sure that the pages follow the manga format (right to left) instead of the western left to right.

It's highly usefull for reading manga comics in the Remarkable 2, although nothing stops you from using it with any other reader.

## Example of use

```bash
manga-converter --file naruto001.cbz
```

## Pending improvements
- Adds support for 
    - `.cbr` files
- Investigate if the initial page of a folder within a cbz should be on it's own
