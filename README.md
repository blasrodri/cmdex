# Command Example (WIP)

This project is an attempt to simplify the life of those users needing some
help when using a command they're unfamiliar with. Instead of having to go to a web engine
or a QA website, just query it here.

## Installation

```bash
cargo install cmdex
```

## Basic Examples

Find all examples of the `find` command
```bash
cmdex find

find - Find files in current working dir with *.txt extension and replace string inplace with sed
Platforms: all
find $(pwd) -name "*.txt" -exec sed -i 's/foo/bar/g {} \;'
Authors: Blas Rodriguez Irizar <rodrigblas@gmail.com>

find - Find all the files whose name is foo.txt in a current working directory
Platforms: all
find . -name foo.txt
Authors: Blas Rodriguez Irizar <rodrigblas@gmail.com>
```

Find all examples that match a certain query on the description of the command example
```bash
cmdex -q="shutdown now"

shutdown - Shutdown your computer right now
Platforms: all
sudo shutdown now
Authors: Blas Rodriguez Irizar <rodrigblas@gmail.com>
```

## Goals
- CLI Support 
- HTTP Support 
- Fuzzy search
- User voting the effectiveness of an example

## Contribute
To add more examples, submit a PR. The only requirement is that the example does not exist.
The _database_ is hosted in json files in the folder called `examples-data`. Add yours, inside
a directory named like the command you're trying to include.