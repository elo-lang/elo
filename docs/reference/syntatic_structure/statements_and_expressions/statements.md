r[statement]
# Statements

r[statement.syntax]
> **<sup>Syntax</sup>**\
> _Statement_ :\
> &nbsp;&nbsp; &nbsp;&nbsp; `";"`\
> &nbsp;&nbsp; | [_Item_]\
> &nbsp;&nbsp; | [_LetStatement_]\
> &nbsp;&nbsp; | [_VarStatement_]\
> &nbsp;&nbsp; | [_ExpressionStatement_]\

r[statement.decl]
## Declaration statements

r[statement.let]
### `let` statements

r[statement.let.syntax]
> **<sup>Syntax</sup>**\
> _LetStatement_ :\
> &nbsp;&nbsp; `"let"` [_PatternNoTopAlt_]
>     ( `:` [_Type_] )<sup>?</sup> (`=` [_Expression_] ) <sup>?</sup> (`";"`) <sup>?</sup>

r[statement.let]
### `var` statements

r[statement.var.syntax]
> **<sup>Syntax</sup>**\
> _VarStatement_ :\
> &nbsp;&nbsp; `"var"` [_PatternNoTopAlt_]
>     ( `:` [_Type_] )<sup>?</sup> (`=` [_Expression_] ) <sup>?</sup> (`";"`) <sup>?</sup>

r[statement.expr]
## Expression statements

r[statement.expr.syntax]
> **<sup>Syntax</sup>**\
> _ExpressionStatement_ :\
> &nbsp;&nbsp; &nbsp;&nbsp; [_ExpressionWithoutBlock_][expression] `";"`<sup>?</sup>\
> &nbsp;&nbsp; | [_ExpressionWithBlock_][expression] `";"`<sup>?</sup>
