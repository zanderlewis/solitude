# Solitude
Simple programming language for beginners.

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
  - [Features](#features)
  - [License](#license)

## Installation
```bash
git clone https://github.com/zanderlewis/solitude.git
cd solitude
gcc -o solitude solitude.c
```

## Usage
```bash
./solitude <filename>
```

## Example
```solitude
x=5
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
```

### Functions
```solitude
func greet Hello!
call greet
```

### Comments
```solitude
. I am a comment
```

### User Input
```solitude
input x Enter a number for x:
$x
```

## Features
- Simple syntax
- Easy to learn
- Small codebase
- Written in C

## License
Solitude is licensed under the [APACHE-2.0](https://www.apache.org/licenses/LICENSE-2.0) license. Please see the [`LICENSE`](LICENSE) file for more information.
