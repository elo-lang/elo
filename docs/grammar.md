# The Elo Programming Language Grammar

This document describes the syntax accepted by Elo's parser.

## Notation

The grammar is expressed as a series of **definitions**.

Each definition consists of one or more **rules**.

A **rule** is a syntax specification applying the notation described below or the composition of a single lexical unit (or token).

This document uses the following notation:

1. **Bold text** denotes the name of the definition being defined at that
   point in the document.
1. *Italic text* denotes a reference to a definition that is elsewhere in this document.
1. A line beneath a definition name is one admissible
   rule of that definition. When more than one such line is present,
   exactly one of them must match — each rule in a definition is
   one exclusive alternative.
1. Text set in `monospace` denotes a terminal: a value that must appear in
   the input exactly as written — a keyword, a punctuation mark, or a
   symbol.
1. Text enclosed in single quotes, as in 'this', denotes a condition on
   the input that cannot be written as a syntax rule, described in natural language.
1. ( and ) enclose a group of symbols for the sole purpose of applying
   one of the operators below to the group as a whole. Parentheses carry
   no meaning on their own.
1. [ and ] enclose an optional sequence: the enclosed text may
   appear zero times or exactly once.
1. A group followed by +, as in (*A*)+, must appear one or more times.
1. A group followed by \*, as in (*A*)\*, may repeat zero or more times. Equivalent to ([*A*])+.
1. Within a group, (*A* | *B*) denotes a choice between *A* and *B*. This
   form is reserved for a short choice nested inside a single line; a
   choice between complete top-level forms is always written as
   separate lines.
1. Text within curly braces (like in {*Text*}) characters match any possible rule except for the Text itself.
   For example, {`X`} matches any possible Unicode character except for the literal X character. 
1. Two rule texts separated by the & character share the same lexical unit (or token).
1. Two values inside an alternation group separated by ... - like in (*A* | ... | *B*) - represent an **inclusive** range between the two values, in which their ommited components are defined by interpretative means.
  For example: (`0` | ... | `4`) represents a clear literal character number range from 0 up until 4. An expanded version would be (`0` | `1` | `2` | `3` | `4`).
1. **Bold text** followed by → and a single rule on the same line is shorthand for a definition that has only one admissible rule — it is equivalent to writing the definition name on its own line followed by that one rule beneath it. This form may only be used when the definition has exactly one rule; as soon as a second alternative is needed, the definition must be written in the full block form instead.

---

## Lexical specification

> **DIGIT**
>
> &nbsp;&nbsp;&nbsp;&nbsp;(`0` | ... | `9`)

> **COUNTINGDIGIT**
>
> &nbsp;&nbsp;&nbsp;&nbsp;(`1` | ... | `9`)

> **ALPHA**
>
> &nbsp;&nbsp;&nbsp;&nbsp;(`a` | ... | `z`)
>
> &nbsp;&nbsp;&nbsp;&nbsp;(`A` | ... | `Z`)

> **ALPHANUM**
>
> &nbsp;&nbsp;&nbsp;&nbsp;(_ALPHA_ | _DIGIT_)

> **IDENTIFIER**
>
> &nbsp;&nbsp;&nbsp;&nbsp;(_ALPHA_ | `_`) & (_ALPHANUM_ | `_`)*

> **INTEGER**
>
> &nbsp;&nbsp;&nbsp;&nbsp;_COUNTINGDIGIT_ & (_DIGIT_ | `_`)*
>
> &nbsp;&nbsp;&nbsp;&nbsp;`0b` & (`0` | `1` | `_`)+
>
> &nbsp;&nbsp;&nbsp;&nbsp;`0o` & ((`0` | ... | `7`) | `_`)+
>
> &nbsp;&nbsp;&nbsp;&nbsp;`0x` & (_DIGIT_ | (`a` | ... | `f`) | (`A` | ... | `F`) | `_`)+

> **FLOAT**
>
> &nbsp;&nbsp;&nbsp;&nbsp;_COUNTINGDIGIT_ & (_DIGIT_ | `_`)\* & `.` & _DIGIT_ & (_DIGIT_ | `_`)*

> **StringLiteral**
>
> &nbsp;&nbsp;&nbsp;&nbsp;`"` & ({`"`})\* & `"`

> **StrLiteral**
>
> &nbsp;&nbsp;&nbsp;&nbsp;`'` & ({`'`})\* & `'`

> **CharacterLiteral**
>
> &nbsp;&nbsp;&nbsp;&nbsp;`` ` `` & {`` ` ``} & `` ` ``

Like whitespace, a *COMMENT* is discarded before parsing and is not
referenced anywhere else in this grammar.

> **BINARYOP**
>
> &nbsp;&nbsp;&nbsp;&nbsp;(`=` | `+=` | `-=` | `*=` | `/=` | `%=` | `&=` | `|=` | `^=` | `==` | `!=` | `<` | `>` | `<=` | `>=` | `&&` | `||` | `^` | `|` | `&` | `+` | `-` | `*` | `/` | `%` | `<<` | `>>`)

> **UNARYOP**
>
> &nbsp;&nbsp;&nbsp;&nbsp;(`&` | `!` | `~` | `-` | `*`)

---

## Program

> **Program**
>
> &nbsp;&nbsp;&nbsp;&nbsp;(*Node*)\*

> **Node**
>
> &nbsp;&nbsp;&nbsp;&nbsp;*Statement*

### Field & list constructs

> **TypedField**
>
> &nbsp;&nbsp;&nbsp;&nbsp;*IDENTIFIER* `:` *Type*

> **TypedFields**
>
> &nbsp;&nbsp;&nbsp;&nbsp;[ *TypedField* (`,` *TypedField*)* [`,`] ]

> **EnumVariants**
>
> &nbsp;&nbsp;&nbsp;&nbsp;[ *IDENTIFIER* (`,` *IDENTIFIER*)\* [`,`] ]

Elo's enums are C-like: a variant is a bare name and carries no associated
data. There is no sum-type / tagged-union form here.

> **ExternParams**
>
> &nbsp;&nbsp;&nbsp;&nbsp;`...`
>
> &nbsp;&nbsp;&nbsp;&nbsp;[ *TypedField* (`,` *TypedField*)\* [`,` `...`] ]

## Statement

> **StatementEnd**
>
> &nbsp;&nbsp;&nbsp;&nbsp;`;`
>
> &nbsp;&nbsp;&nbsp;&nbsp;`}`
>
> &nbsp;&nbsp;&nbsp;&nbsp;'line feed character ASCII 10'
>
> &nbsp;&nbsp;&nbsp;&nbsp;'the end of the input'

> **Statement**
>
> &nbsp;&nbsp;&nbsp;&nbsp;*StatementBody* *StatementEnd*

*StatementEnd* is applied exactly once, uniformly, at this single wrapping
point. None of the individual statement definitions below consume their
own *StatementEnd* — that responsibility lives here only, so every kind of
statement (declarations and control flow alike) is terminated the same
way.

> **StatementBody**
>
> &nbsp;&nbsp;&nbsp;&nbsp;*StructDecl*
>
> &nbsp;&nbsp;&nbsp;&nbsp;*EnumDecl*
>
> &nbsp;&nbsp;&nbsp;&nbsp;*ConstDecl*
>
> &nbsp;&nbsp;&nbsp;&nbsp;*FunctionDecl*
>
> &nbsp;&nbsp;&nbsp;&nbsp;*ExternFunctionDecl*
>
> &nbsp;&nbsp;&nbsp;&nbsp;*VarDecl*
>
> &nbsp;&nbsp;&nbsp;&nbsp;*LetDecl*
>
> &nbsp;&nbsp;&nbsp;&nbsp;*IfStmt*
>
> &nbsp;&nbsp;&nbsp;&nbsp;*WhileStmt*
>
> &nbsp;&nbsp;&nbsp;&nbsp;*ReturnStmt*

> **IfStmt**
>
> &nbsp;&nbsp;&nbsp;&nbsp;`if` *Expression* *Block* [ `else` (*IfStmt* | *Block*) ]

> **WhileStmt**
>
> &nbsp;&nbsp;&nbsp;&nbsp;`while` *Expression* *Block*

> **ReturnStmt**
>
> &nbsp;&nbsp;&nbsp;&nbsp;`return` [*Expression*]

> **Block**
>
> &nbsp;&nbsp;&nbsp;&nbsp;`=>` *Node*
>
> &nbsp;&nbsp;&nbsp;&nbsp;`{` (*Node*)\* `}`

> **StructDecl**
>
> &nbsp;&nbsp;&nbsp;&nbsp;`struct` *IDENTIFIER* `{` *TypedFields* `}`

> **EnumDecl**
>
> &nbsp;&nbsp;&nbsp;&nbsp;`enum` *IDENTIFIER* `{` *EnumVariants* `}`

> **ConstDecl**
>
> &nbsp;&nbsp;&nbsp;&nbsp;`const` *IDENTIFIER* `:` *Type* `=` *Expression*

> **FunctionDecl**
>
> &nbsp;&nbsp;&nbsp;&nbsp;`fn` *IDENTIFIER* `(` *TypedFields* `)` [`:` *Type*] *Block*

> **ExternFunctionDecl**
>
> &nbsp;&nbsp;&nbsp;&nbsp;`extern` `fn` *IDENTIFIER* `(` *ExternParams* `)` [`:` *Type*]

> **VarDecl**
>
> &nbsp;&nbsp;&nbsp;&nbsp;`var` *IDENTIFIER* `=` *Expression*

> **LetDecl**
>
> &nbsp;&nbsp;&nbsp;&nbsp;`let` *IDENTIFIER* `=` *Expression*


## Types

> **Type**
> &nbsp;&nbsp;&nbsp;&nbsp;`(` *Type* `)`
>
> &nbsp;&nbsp;&nbsp;&nbsp;*FunctionType*
>
> &nbsp;&nbsp;&nbsp;&nbsp;*NamedType*
>
> &nbsp;&nbsp;&nbsp;&nbsp;*PointerType*
>
> &nbsp;&nbsp;&nbsp;&nbsp;*ArrayType*
>
> &nbsp;&nbsp;&nbsp;&nbsp;*SliceType*
>
> &nbsp;&nbsp;&nbsp;&nbsp;*TupleType*

> **FunctionType**
>
> &nbsp;&nbsp;&nbsp;&nbsp;`fn` `(` [*TypeList*] `)` [`:` *Type*]

> **NamedType**
>
> &nbsp;&nbsp;&nbsp;&nbsp;*IDENTIFIER* [`<` *TypeList* `>`]

> **PointerType**
>
> &nbsp;&nbsp;&nbsp;&nbsp;`*` [`mut`] *Type*

> **ArrayType**
>
> &nbsp;&nbsp;&nbsp;&nbsp;`{` *Type* `;` *INTEGER* `}`

> **SliceType**
>
> &nbsp;&nbsp;&nbsp;&nbsp;`{` *Type* `}`

> **TupleType**
>
> &nbsp;&nbsp;&nbsp;&nbsp;`(` *Type* (`,` *Type*)+ `)`

> **TypeList**
>
> &nbsp;&nbsp;&nbsp;&nbsp;*Type* (`,` *Type*)\*

## Expressions

### Operator precedence

*BINARYOP* and *UNARYOP* are flat token classes — the grammar itself does
not encode precedence. It is documented here instead:

| Level | Operators | Associativity |
|---|---|---|
| 9 (highest) | unaries: `&`, `!`, `~`, `-`, `*` | right |
| 8 | `<<` `>>` | left |
| 7 | `*` `/` `%` | left |
| 6 | `+` `-` | left |
| 5 | `^` `\|` `&` | left |
| 4 | `&&` `\|\|` | left |
| 3 | `<` `>` `<=` `>=` | left |
| 2 | `==` `!=` | left |
| 1 (lowest) | `=` and compound assignment | left |

Postfix forms (`.`, `(...)`, `[...]`, `as`) bind tighter than any entry in
this table, including unary. The compound assignment operators (`+=`,
`-=`, etc.) are placed at level 1 alongside `=` on the assumption that
they share its precedence — the source table only lists bare `=`.

> **Expression**
>
> &nbsp;&nbsp;&nbsp;&nbsp;*Primary*
>
> &nbsp;&nbsp;&nbsp;&nbsp;*Expression* *BINARYOP* *Expression*
>
> &nbsp;&nbsp;&nbsp;&nbsp;*Expression* `.` (*INTEGER* | *IDENTIFIER*)
>
> &nbsp;&nbsp;&nbsp;&nbsp;*Expression* `(` [ *ExpressionList* ] `)`
>
> &nbsp;&nbsp;&nbsp;&nbsp;*Expression* `[` *Expression* `]`
>
> &nbsp;&nbsp;&nbsp;&nbsp;*Expression* `as` *Type*
>
> &nbsp;&nbsp;&nbsp;&nbsp;*UNARYOP* *Expression*

*UNARYOP* and *BINARYOP* deliberately share some symbols (`&`, `-`, `*`).
This is not a conflict: the same symbol in prefix position and in infix
position are different operators that happen to share a spelling,
disambiguated by syntax position.

> **Primary**
>
> &nbsp;&nbsp;&nbsp;&nbsp;**Numeric** → (*INTEGER* | *FLOAT*)
>
> &nbsp;&nbsp;&nbsp;&nbsp;**TupleLiteral** → `(` *Expression* `,` *ExpressionList* `)`
>
> &nbsp;&nbsp;&nbsp;&nbsp;**StructLiteral** → *IDENTIFIER* `{` *Fields* `}`
>
> &nbsp;&nbsp;&nbsp;&nbsp;**BoolLiteral** → (`true` | `false`)
>
> &nbsp;&nbsp;&nbsp;&nbsp;**ArrayLiteral** → `{` *ExpressionList* `}`
>
> &nbsp;&nbsp;&nbsp;&nbsp;`(` *Expression* `)`
>
> &nbsp;&nbsp;&nbsp;&nbsp;*CharacterLiteral*
>
> &nbsp;&nbsp;&nbsp;&nbsp;*StringLiteral*
>
> &nbsp;&nbsp;&nbsp;&nbsp;*StrLiteral*

### Field & list constructs

> **Field**
>
> &nbsp;&nbsp;&nbsp;&nbsp;*IDENTIFIER* `:` *Expression*

> **Fields**
>
> &nbsp;&nbsp;&nbsp;&nbsp;[ *Field* (`,` *Field*)\* [`,`] ]

> **ExpressionList**
>
> &nbsp;&nbsp;&nbsp;&nbsp;*Expression* (`,` *Expression*)\* [`,`]
