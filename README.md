# Solitude
Simple programming language that supports threading.

## Table of Contents
- [Solitude](#solitude)
  - [Table of Contents](#table-of-contents)
  - [Installation](#installation)
  - [Usage](#usage)
  - [Example](#example)
  - [Syntax](#syntax)
    - [Variables](#variables)
    - [Print](#print)
    - [If](#if)
    - [Functions](#functions)
    - [Comments](#comments)
    - [User Input](#user-input)
    - [Threading](#threading)
  - [Features](#features)
  - [License](#license)

## Installation
```bash
git clone https://github.com/zanderlewis/solitude.git
cd solitude
```

## Usage
```bash
cargo run <filename>
```

## Example
```solitude
var x=5
The value of x is: $x
```

## Syntax
Solitude has very simple syntax. Here is a complete list:

### Variables
```solitude
var x=5
```
### Print
```solitude
Hello World

var x=5
$x
```

### If
```solitude
if x>5
x is greater than 5
fi
```

### Functions
```solitude
func greet
Hello!
cnuf

call greet
```

### Comments
```solitude
. I am a comment
```

### User Input
```solitude
. The \x20 is a space character
input x -> Enter a number for x:\x20
$x
```

### Threading
```solitude
!!
call func1
call func2
??
```

## Features
- Simple syntax
- Easy to learn
- Small codebase
- Fast
- Built-in threading
- Written in Rust

## License
Solitude is licensed under the [Apache-2.0](https://www.apache.org/licenses/LICENSE-2.0) license. Please see the [`LICENSE`](LICENSE) file for more information.
