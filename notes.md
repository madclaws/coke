# Notes during development

## 2022-03-26 03:11:42

### Scanning
Reading a source text as a group of characters and then separate that into meaningful grammar of the language.
The grammar words are called **Tokens**

## 2022-03-27 01:39:28
For reading input line by line, like in repl.

```
let mut buffer = String.new();
std::io.Write::stdin().readLine(&mut buffer)
```

💡 **static / global variables are stored in data segment of the programs memory, not in heap nor in stack.**

### Global variables
Right now we need a thread safe global variable for HAD_ERROR_MUTEX.
We can have a Mutex<bool> type var, but it can't be used for a static variable.
So we use Lazy initialization too using once_cell

## 2022-03-27 17:57:22

Lexemes - group of characters that makes sense in our lang
```
let language = "lox"
```
ex `let` makes sense, but `gua` doesn't

How lexemes makes sense?. if they come under a `TokenType`. 