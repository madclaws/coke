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

## 2022-04-17 03:12:04

- Tests cases added for keywords and identifiers.
- Bug fixes.

Next: Adding Block comments `/* */`

## 2022-04-18 02:01:39

- The lexical grammars of Python and Haskell are not regular. What does that mean, and why aren’t they?

    - Regular in the sense, i guess those that can be expressed by regular expressions.
    - So there might be some grammar rule that can't be expressed with regular expressions.
    - multiple derivations for a grammar?
    - context-free grammar. Grammars that use memory (for context)
    - https://en.wikipedia.org/wiki/Context-free_grammar
    - https://en.wikipedia.org/wiki/Nondeterministic_finite_automaton

## 2022-04-19 00:19:18

- Added block comments.
- Clippy fixes

Next: `Representing Code`

## 2022-04-19 11:43:50

- Representing code as a tree in an order of syntactic grammar
- Regualar grammar can't represent nested syntax, as it requires a memory stack, which finite automaton
    isn't.
- There are 2 types of grammars - Lexical grammar & syntactic grammar.
- lexical grammar
    - each alphbaet is `character`
    - formed strings are lexemes
    - implemented by scanner
- syntactic grammar
    - each alphabet is `token`
    - formed strings are `expressions`
    - implemented by parser.

- formal languages make sure that the generated strings from the alphabets are valid or not.

## 2022-04-21 01:01:26

Rules for grammars

- Strings are derived from certain rules, so its called `derivations`.
- Rules are called `productions`, as they prodcuce string.
- A productions has `head` and `body`
- Both of them are symbols
- Symbols are of 2 types
    - terminal
        - These are basically lexemes/tokens. 
        - These don't expand again.
    - non-terminal
        - These are references to other productions or self-referencing.
        - These expand.
- Genrally we use `Backus-Naur` form to represent grammar.

Next - Enhancing our notation

## 2022-04-22 02:07:32

We can add more symbols other than terminals & non-terminal like,
- `|` (pipe) for enum like behaviour
- `()` for grouping the symbols
- `+`, so that previous symbol should appear once more
- `*` - previous symbol can be repeated >= 0 times
- `?` - previous symbol can appear 0 or 1

Next - A Grammar for Coke expressions

## 2022-04-23 01:36:19

- We will implement a subset of grammar first, parse it and interpret it.
- First version Grammars are
    - Literals
    - Binary operators
    - Unary operators
    - paranthesis.
- lexems whose length can vary are expressed in CAPITAL in grammar, ex String, Number

Next - Implementing Syntax Trees

## 2022-04-24 02:12:30

- In AST, each node is an expression.
- We have to create some structs
    - Expression, and all production structs.

- Added some generic framework for the `expr.rs`.

Todo:
    - Manually add structs of types and complete it .
    - Check if this can be generated by `cargo single`
## 2022-04-25 03:39:03
Backus Naur form for subset of Coke grammar

```
expression     → literal
               | unary
               | binary
               | grouping ;

literal        → NUMBER | STRING | "true" | "false" | "nil" ;
grouping       → "(" expression ")" ;
unary          → ( "-" | "!" ) expression ;
binary         → expression operator expression ;
operator       → "==" | "!=" | "<" | "<=" | ">" | ">="
               | "+"  | "-"  | "*" | "/" ;
```

Structures for expressions are done.

Next: Working with Trees

## 2022-04-26 23:46:58

- The expression problem
    - We have type and methods associated with it.
    - These methods are getting called when the syntax tree get executed by interpreter
    - ex: for`Binary` type, there will be a method `interpret`
    - In object oreinted way, for each type class, we have to implement the methods.
    - But if we want to add a new method, we have to go to all classes and add the implementation.
    - In FP, its reverse:
        - Each functions like `interpret` will implement all its cases for all the types.
        - So adding new funcitons are easy.
        - But adding a new type means  go to interpret and add a new case for new type. 

Next: Visitor pattern

## 2022-04-28 00:31:01

- Visitor pattern
    - From a OOP perspective,
        - We add an interface visitor that will define functions, that take the  subclass objects.
        - visitDog(Dog dog), where Dog, is the subclass.
    - Then in each subclass, we add a function `accept`, which is overrided from the parent `animal`.
    - `accept` takes a visitor obj, and inside `accept`, we call visitor.visitDog(this)

## 2022-04-28 21:24:05

- Visitor for expressions
    - Defined Visitor trait.

Next: Implement accept function in Expression structs

## 2022-04-30 00:13:01

- Added accept fn in Expr struct.

- Need to understand what we want to achieve / how to express the visitor pattern properly in Rust.

## 2022-04-30 02:35:15

- Refreshing up from syntax tree.
    - Expressions are not homogeneous, they have different operands

- OOP with Rust.
    - An interface can be implemented with Trait Objects.
    - functions defined inside a trait have first argument `self`

- Visitor pattern - Add methods to different types/same type of class, without changing the classes


## 2022-05-01 12:43:25

- visitor function heads in AST printer

Next: Complete the body of visit_* functions

## 2022-05-05 02:23:09

- Removed the struct and OOPy implementation of visitor pattern,
- Implemnting using a Expr enum, and all the expressions are its variant.

Next: Implement the code interpreation in AstPrinter

## 2022-05-05 23:32:58

## 2022-05-07 02:03:48

- Added paranthesize fn + interpreter for all 4 expressions

Next: Pretty print expressions

## 2022-05-07 14:41:27

- Writing tests for AstPrinter.

## 2022-05-08 15:49:00

- Parsing expressions
    - Ambiguity and parsing game.
        - From a series of tokens, we have to figure out the grammar used for it
            validate it.
        - We can generate multiple syntax trees for same expression , with the given grammar and that is not good.
        - Operators are left associative and assignment is right associative


 NEXT: Precedence rules in Coke

## 2022-05-10 03:50:44

 - Modified Grammar for handling precedence

 ```
expression     → equality ;
equality       → comparison ( ( "!=" | "==" ) comparison )* ;
comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
term           → factor ( ( "-" | "+" ) factor )* ;
factor         → unary ( ( "/" | "*" ) unary )* ;
unary          → ( "!" | "-" ) unary
               | primary ;
primary        → NUMBER | STRING | "true" | "false" | "nil"
               | "(" expression ")" ;
 ```

 Next: Recursive descent parser

## 2022-05-11 22:06:46

- Top down parser
    - Starts from the root expression and ends at the leaves or terminals
- Bottom-up parser
    - Starts from the innermost expressions and end at the complete one.

- In a top-down parser, we start from the lowest precedence expressions

- The Parser class

Next: Parser implementation continues

## 2022-05-12 02:59:13

- Parser takes a list of Tokens, where as scanner took list of source characters

- isAtEnd, peek, previous

## 2022-05-15 01:57:03

- Each grammer rule become a method in a class
- Equality function 
- Borrow checker fucked..
- Lifetime fuck up, may be have to write Expr in struct than Enum

## 2022-05-17 01:35:01
- Fix reference issue with Expr enum

## 2022-05-17 22:10:52
- Recursive Descent Parser
    - https://www.huy.rocks/everyday/05-08-2022-parsing-recursive-descent-parser
    - Every non-terminal is a function
    - Added MTokenType and MToken
    Next: Adding Parser

## 2022-05-20 12:07:58
- Creating a custom error type
    - The error type is basically enum.
    - This custom type will be the error type of the Result of parser
    - Valid result of parser will be a MoneyNode.
        - A moneynode has Currency and amount

        {
            currency: "$",
            amount: "123"
        }
    - Implementing Display for nice rendering of error

## 2022-05-23 23:15:07

- Lifetime mess up

## 2022-05-24 21:28:50
- Needs a reread of parser part

## 2022-05-25 22:06:08

### Parsing expressions
- Given a token, we map them to the terminals to figure out which rules could have generated it.
- 6 / 3 - 1, multiple ASTs can be generated and some can be wrong too acc to rules.
- binary operators are left associative.
- assignments are right associative.
- We will apply precedence like in C to the grammar
    - ==, !=
    - comparison,  > | < etc..
    - Term, + | -
    - Factor, / | *
    - Unary, - ! - right associative

- new grammar

```
expression -> equality
equality -> comparison (("=" | "!=") comparison)*
comparison -> term ((">" | "<" | ">=" | "<=") term)*
term -> factor (("+" | "-") factor) *;
factor -> unary (("/" | "*") unary) *; // for unlimited expressions
unary -> ("!" | "-") unary | primary;
primary -> NUMBER | STRING | "nil" | "true" | "false" | "(" expression ")";
```

- Each prodcution should call itself and its higher precedence
- We are not making left recursive parser.
- If an expression contains 3 + 4

## 2022-05-27 21:56:14

- Recursive descent parsing
    - They are top bottom parser, as they start from lower precedence, full expression
        to the terminals

- The parser class
- equality
    - comparison
    - while loop for equality ops, 
        if eqality ops, then we create a binary expression with right comparison too
    
    in an expression like this `a == b == c == d == e`
    all this will be inside the while loop
    expr = Binary(expr, operator, right)
    previous binary operator will be the left expression in the next iteration.

    - If no equality ops, then only `comparison` will get called.

- Comparison is same as equality, baring the tokens
- Same as above for term and factor.
- For unary, its if ops is unary, then new Unary(), else call primary()
- For primary, we just check the ifs and return a new Lit()
- Next is syntax errors
- But before that, we have reread and write the parser till here?
- No, we can test the parser, without the error handling at first..
- May the borrow checker be with me!!

## 2022-05-28 22:38:55

### Crust of Rust: Smart Pointers and interior mutability

- Interior mutability with Cells
    - Shared mutable containers O.O, which is not possible by default in Rust as brrow checker wont allow.

`use std::cell::Cell` - The saviour of COKE

- [NEW] - Use `std::cell::Cell` for sharable mutable containers, with compile time safety
- Next: Syntax Errors, by handling the grouping expression

## 2022-05-30 09:12:36
- [DONE] - Wrote a test to check if parser can create an AST for simple math expression `2 + 3`
### Syntax errors

- Compiler's job
    - Convert the tokens to AST
    - Neat error handling.
- Error handling requirements of a parser
    - Detect & report error
    - Shouldn't crash or stuck
    - Fast
    - Reporting all the errors
    - minimizing cascading errors- ie errors that happen due to parser confused due to first error.

- Panic mode recovery
    - Parser enters panic mode, as soons as there is an error on one token.
    - synchronization
        - After panic mode, parser need to make the its state correct for the next token
        - We make a syncronization point, which is from where parser can again start parsing.
        - This means we discard all the token before it in the token stream
        - Errors in the skipped tokens will not be examined by the parser
        - This also means that we skip the cascade false error that could have happen too
        - Parsers do the synchronization in statements, which we don't have yet, we only have basic expressions.

- Added ParseResult type for handling parse error

## 2022-06-01 22:30:21

- Added errorv2 and refactor

## 2022-06-03 00:21:18

- Error productions are nicer.
- Added synchronize function

## 2022-06-03 20:50:32

- Added ExpectExpression variant to the parseError

## 2022-06-05 00:33:35

- Added parser to main wip
    - mutability return issue in scanner.rs

## 2022-06-06 21:30:58
- Finally mathematical simple parser is working.
- [TODO] - Manage errors correctly.
    -[DONE] - Printing unexpected token error
    - [TODO] - Need extensive error report

## 2022-06-08 22:17:15
- [DONE] - Making a group expression , if only consume return is OK

## 2022-06-09 22:42:10

- [DONE] - Removed redundant error function

## 2022-06-10 22:05:41

- [DONE] - Why REPL is exiting on consume error
    - Because we set HAD_ERROR_MUTEX to true in consume, which lead to the exit.

- [TODO] - Grammar for ternary operator

## 2022-06-10 22:43:59

- Evaluating expressions
    - We will be executing the syntax tree itself.s
    - Use the same visitor pattern to compute the values
    - Coke literal values are in the end Rust's types.
    - Interpreter struct with Visitor trait
    - The challenge is in Java all the coke values can be covered by `Object`

- Evaluating literals
- Evaluating Paranthesis
    - evaluate the inner expression recursively.
- Evaluating unary expression
    - evaluate(right expression)
    - right expression is casted according to the symbol.
    - Interpreter is post-order-traversal.
    - Check the truthy value
- Truthiness and falness
    - Dynamic langs partition the data into truthy and falsy.
    - Coke follows, `false` and `nil` are falsy, rest everything truthy
- Evaluating Binary operators
    - evaluate expressions on the basis of operator type.
    - For `PLUS` we check the type for Number or string , for addition on concatenation