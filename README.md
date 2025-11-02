[![release](https://img.shields.io/badge/v0.0.1-a6e3a1?style=for-the-badge&labelColor=1e1e2e&logoColor=a6e3a1&label=release)](https://github.com/AndyLocks/irnix/releases/tag/irnix-v0.0.1)

[![en_docs](https://img.shields.io/badge/EN_Documentation-89b4fa?style=for-the-badge&labelColor=1e1e2e&logoColor=cdd6f4&logo=mdbook)](https://andylocks.github.io/irnix-docs/en/book/index.html)
[![ru_docs](https://img.shields.io/badge/RU_Documentation-89b4fa?style=for-the-badge&labelColor=1e1e2e&logoColor=cdd6f4&logo=mdbook)](https://andylocks.github.io/irnix-docs/ru/book/index.html)

**[More detailed documentation](https://andylocks.github.io/irnix-docs/en/book/index.html)** is also available, which is also [available in Russian language](https://andylocks.github.io/irnix-docs/ru/book/index.html).

# What Is **Irnix**

**Irnix** was inspired by the [Object‑Oriented Programming paradigm](https://en.wikipedia.org/wiki/Object-oriented_programming), therefore it can be described as a _system for organizing objects_, which turns the file system into objects with methods and contracts.  
This allows scripts to be organized, method calls to be validated, a single call point to exist, and object implementations to be easily swapped.

**Irnix** helps create a strict structure through contract‑checking mechanisms (method signatures), thereby ensuring correct method invocation on objects and helping avoid erroneous behavior in the pipeline.

# Quick Start

Create an `object` directory inside `~/.local/share/irnix` and add a file named `method` there:

```
~/.local/share/irnix
└── object
    └── method
```

Contents of the `method` file:

```bash
#!/bin/bash

echo "Hello $1"
```

Give the file executable permissions:

```bash
chmod +x ~/.local/share/irnix/object/method
```

Invoke the method:

```bash
irnix e object.method -- World!
```

Output:

```
Hello World!
```
