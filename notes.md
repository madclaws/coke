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

## 2022-04-01 21:39:41
### Where are we?
- We defined tokenTypes as an Enum

Token struct
    - TokenType
    - lexeme (string)
    - line - line no in u32
    - literal, this is an `Object` in Java, will figure this out later
        - This is basically a runtime version of the literal token such as number/string, runtime version means if a literal is "4" as a lexeme, then we can convert to an integer 4.

```
`token_type::TokenType` doesn't implement `std::fmt::Display`

Can use #[derive(Debug)] to print enums
```

- `#[allow(dead_code)]` for turning of unused code warning

Things done,
    - Token module and struct
    - new and to_string functions in Token struct.

## 2022-04-03 01:35:49

Lexical grammar - Rules that groups character to a lexeme..

Scanner module
- Consistes of source (String) and list of Tokens
- Scanner loop - iterates over the source and add tokens to list.
    start - start pos of lexeme
    current - current pos of lexeme
    line = current line no (for error reporting)
    loop ends when current > length of source. 
- We add an EOF token at the end of token list.

Today - Basic Scanner Module Done.

Coming up - [Recogonizing Lexemes](http://www.craftinginterpreters.com/scanning.html#recognizing-lexemes)
