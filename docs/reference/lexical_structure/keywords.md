r[lex.keywords]
# Keywords

r[lex.keywords.strict.list]
> **<sup>Lexer:<sup>**\
> KW_AS             : `as`\
> KW_ASYNC          : `async`\
> KW_AWAIT          : `await`\
> KW_BREAK          : `break`\
> KW_CATCH          : `catch`\
> KW_CONST          : `const`\
> KW_CONTINUE       : `continue`\
> KW_CRATE          : `crate`\       # RECHECK\
> KW_DYN            : `dyn`\
> KW_ELSE           : `else`\
> KW_ENUM           : `enum`\        # OR `data`\
> KW_EXTERN         : `extern`\      # RECHECK\
> KW_EXTN           : `extn`\
> KW_FALSE          : `false`\
> KW_FN             : `fn`\
> KW_FOR            : `for`\
> KW_GEN            : `gen`\
> KW_GIVE           : `give`\        # EXPR.`give` OR `give` EXPR\
> KW_IF             : `if`\
> KW_IMPL           : `impl`\
> KW_IN             : `in`\
> KW_LEASE          : `lease`\       # EXPR.`lease` OR `lease` EXPR\
> KW_LEASED         : `leased`\      # `let` PAT: `leased` TYPE = ...\
> KW_LET            : `let`\
> KW_LOOP           : `loop`\
> KW_MACRO          : `macro`\
> KW_MATCH          : `match`\
> KW_MOD            : `mod`\
> KW_MOVE           : `move`\        # EXPR.`m̀ove` OR `move` EXPR\
> KW_MUT            : `mut`\
> KW_ON             : `on`\
> KW_OF             : `of`\          # EXPR `of` Trait.METH\
> KW_OWN            : `own`\         # `let` PAT: `own` TYPE = ...\
> KW_PUB            : `pub`\
> KW_REF            : `ref`\         # EXPR.`ref` OR `ref` EXPR\
> KW_RETURN         : `return`\      # OR `ret`\
> KW_SELFVALUE      : `self`\
> KW_SELFTYPE       : `Self`\
> KW_SHARE          : `share`\       # EXPR.`share` OR `share` EXPR\
> KW_SHARED         : `shared`\      # `let` PAT: `shared` TYPE = ...\
> KW_STATIC         : `static`\
> KW_STRUCT         : `struct`\
> KW_SUPER          : `super`\
> KW_TRAIT          : `trait`\
> KW_TROW           : `trow`\
> KW_TRUE           : `true`\
> KW_TRY            : `try`\
> KW_TYPE           : `type`\
> KW_UNSAFE         : `unsafe`\      # RECHECK\
> KW_USE            : `use`\
> KW_WHERE          : `where`\
> KW_WHILE          : `while`\
> KW_YIELD          : `yield`\

r[lex.keywords.permissions.table]
## Permissions
| Type Permission     | Permission |
|---------------------|------------|
| 🗑️my, own, 🗑️given  | give, move |
| 🗑️our, shared       | share      |
| ®refd               | ref        |
| leased              | lease      |
