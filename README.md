# Magick-MCP

A MCP server which facilitates usage of  [ImageMagick](https://imagemagick.org/)

# Installation

```bash
cargo install magick-mcp
```

# Tools

This MCP sever supports the following tools:

- check
- magick
- func_save
- func_execute
- func_list

## Check Tool

The check tool simply validates that imagemagick is installed and ready to use.

## Magick Tool

The magick tool execute imagemagick commands. For example:

```
Use magick to turn test.jpg into a gray image.
```

Will generate a command like this:
```bash
test.jpg -colorspace Gray test-gray.jpg
```

## Save Functions Tool

The func_save tool will save a series of imagemagick commands as a reusable function. For example:

```
Use magick to create a function that will generate two versions of an input image:
- gray
- inverted
```

Will generate a function like this:
```
Name: gray_and_inverted
Commands:
  - $input -colorspace Gray $input-gray.jpg
  - $input -negate $input-inverted.jpg
```

> [!NOTE]
> `$input` can be used to represent the input file. It will be replaced with the actual input file during function execution.

##  Execute Function Tool

The func_execute will execute a previously saved function. The agent will supply the name of the function and the input file.

## List Functions Tool

The func_list tool will simply list out previously saved tools.

# Under The Hood

When executing imagemagick commands this MCP server will invoke magick via the shell. When workspace is provided we will set that as the working directory.

> [!NOTE]
> The server clears all environment variables with the exception of the path.