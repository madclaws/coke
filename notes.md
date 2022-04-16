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

## 2022-04-04 02:19:39

- Scanning single character tokens and adding it to the token list.
- **We are not handling, the lexemes that doesnt have a token type**

Coming up - [Lexical errors](http://www.craftinginterpreters.com/scanning.html#lexical-errors)

## 2022-04-05 02:11:54

Calling pub function of `main.rs` from other modules?

- Use `crate::function_name`

- Basic Test cases for scanner - Done

Coming up - [Operators](http://www.craftinginterpreters.com/scanning.html#operators)

## 2022-04-05 20:00:07

Handling more than one character lexemes
ex: if we get `!`, then we check the next character is `=`, if yes, then
inc the current and token will be `BangEqual`.

## 2022-04-06 02:31:07

- Added scanner for more than 1 character lexeme.

Coming up - [Longer Lexemes](http://www.craftinginterpreters.com/scanning.html#longer-lexemes)

## 2022-04-07 11:18:33

Added scanner for `/` and `//`

- Todo next - Add test for `/` and `//`

## 2022-04-07 21:19:17

- Test for longer lexeme - done
- Handle whitespace characters

Coming upp - [String Literals](http://www.craftinginterpreters.com/scanning.html#string-literals)

## 2022-04-08 21:00:42

- String literal handling - done.

Todo:
- `scan_tokens` is messy now
    - The issue is sometimes we have to return crate::error message. Fix the return type.
- Add MORE tests for String literals.

## 2022-04-09 21:23:38

- Refactored scanner
- Tests for string literal.

- Whats the fuzz with returning &str in `get_last_string_char`

Coming up - [Number literal](http://www.craftinginterpreters.com/scanning.html#number-literals)

Femo fix test

## 2022-04-11 10:31:53

- Tests for Strings.

## 2022-04-12 01:56:48

Ownership
- The main point of using ownrship is to handle memory allocation in heap and using stack and heap efficiently.

- Operations on Heap are costly, ownership helps us to manage these efficiently.

- Rust wants to focus more on operating on slack than Heap.

- `move` protects us from double-freeing the memory in heap
    - Since when out of scope, `drop` is called. rust makes sure that `drop` is
    called for a memory once.

- During an assignment/function call, either of 2 things can happen, `copy`/`move`.
    copy - if value is in stack
    move - if in heap

- For deep copy (copying the heap instead of stack pts) use `clone`.
    cloning large data is less performant.

- `When a variable that includes data on the heap goes out of scope, the value will be cleaned up by drop unless ownership of the data has been moved to another variable.`
Moving to another variable can be via return too.

## 2022-04-14 12:38:45

- Literal can be now defined as a Literal Enum with either String or f64?
- implemented scanner for numbers.

Coming up - Test cases for numbers

## 2022-04-15 14:22:46

- Added tests for Number literals.

Coming up - **Reserved words and identifiers**

## 2022-04-16 16:37:48

- maximal munch - If a gramatical rule can be used against a keyword and identifier, then we should consume the maximum possible to decide whether it is keyword / identifier.

- Coming up - Testng of keywords and identifiers & footnotes
            