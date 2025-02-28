# Elo Grammar Specification

## Primary
> _Primary_ :  
> &nbsp;&nbsp;&nbsp;&nbsp; _Ident_   
> &nbsp;&nbsp;&nbsp;&nbsp; | _NumberLiteral_  
> &nbsp;&nbsp;&nbsp;&nbsp; | _StringLiteral_  

> _Ident_ :  
> &nbsp;&nbsp;&nbsp;&nbsp; `[A-Za-z_] [A-Za-z0-9_]*`

> _NumberLiteral_ :  
> &nbsp;&nbsp;&nbsp;&nbsp; _DecimalNumber_  
> &nbsp;&nbsp;&nbsp;&nbsp; | _HexNumber_  
> &nbsp;&nbsp;&nbsp;&nbsp; | _OctalNumber_  
> &nbsp;&nbsp;&nbsp;&nbsp; | _BinaryNumber_  

> _StringLiteral_ :  
> &nbsp;&nbsp;&nbsp;&nbsp; `"([^"\\]*(\\.[^"\\]*)*)"`

> _DecimalNumber_ : `0-9`+  
> _HexNumber_ : `'0x' [0-9A-Fa-f]+`  
> _OctalNumber_ : `'0o' [0-7]+`  
> _BinaryNumber_ : `'0b' [0-1]+`  

## Operators

> _BinaryOperator_ :  
> &nbsp;&nbsp;&nbsp;&nbsp; `"+"` | `"-"` | `"*"` | `"/"`  
> &nbsp;&nbsp;&nbsp;&nbsp; | `"|"` | `"&"` | `"<<"` | `">>"`  
> &nbsp;&nbsp;&nbsp;&nbsp; | `"||"` | `"&&"` | `"=="` | `"!="` | `">"` | `"<"` | `"<="` | `">="`  

> _UnaryOperator_ : `"-"` | `"!"` | `"&"`  

## Types

> _Type_ :  
> &nbsp;&nbsp;&nbsp;&nbsp; _NamedType_  
> &nbsp;&nbsp;&nbsp;&nbsp; | _ArrayType_  
> &nbsp;&nbsp;&nbsp;&nbsp; | _PointerType_  
> &nbsp;&nbsp;&nbsp;&nbsp; | _TupleType_  
> &nbsp;&nbsp;&nbsp;&nbsp; | _FunctionPointerType_  

> _NamedType_ : _Ident_ [`"<"` _Type_ [`","` _Type_]* `">"`]  
> _ArrayType_ : `"["` _Type_ `","` _DecimalNumber_ `"]"`  
> _PointerType_ : `"*"` _Type_  
> _TupleType_ : `"("` _Type_ [`","` _Type_]* `")"`  
> _FunctionPointerType_ : `"fn("` [_Type_ [`","` _Type_]*] `")"` [`":"` _Type_]  

## Statements

> _Statement_ :  
> &nbsp;&nbsp;&nbsp;&nbsp; _ExpressionStatement_  
> &nbsp;&nbsp;&nbsp;&nbsp; | _KeywordStatement_  

> _ExpressionStatement_ : _Expression_ _End_  
> _KeywordStatement_ : _LetStatement_ | _FunctionDefinition_ | _StructDefinition_  

> _LetStatement_ : `"let"` _Ident_ `"="` _Expression_ _End_  
> _FunctionDefinition_ : `"fn"` _Ident_ `"("` [_TypedFields_] `")"` [`":"` _Type_] `"{"` [_Statement_]* `"}"` [_End_]  
> _StructDefinition_ : `"struct"` _Ident_ `"{"` _TypedFields_ `"}"` [_End_]  

## Expressions

> _Expression_ :  
> &nbsp;&nbsp;&nbsp;&nbsp; Primary  
> &nbsp;&nbsp;&nbsp;&nbsp; | _BinaryOperation_  
> &nbsp;&nbsp;&nbsp;&nbsp; | _UnaryOperation_  
> &nbsp;&nbsp;&nbsp;&nbsp; | _FunctionCall_ 

> _FunctionCall_ : _Expression_ `"("` [_ExpressionList_] `")"`  
> _BinaryOperation_ : _Expression_ _BinaryOperator_ _Expression_  
> _UnaryOperation_ : _UnaryOperator_ _Expression_  

> _StructInitialization_ : _Ident_ `"{"` [_Fields_] `"}"`  
> _FieldAccess_ : _Expression_ `"."` _Ident_  
> _NamespaceAccess_ : _Ident_ `"::"` _Ident_  

## Other

> _End_ : `"\n"` | `";"` | _EOF_  
> _Fields_ : _Field_ [`","` _Field_]*  
> _TypedFields_ : _TypedField_ [`","` _TypedField_]*  
> _Field_ : _Ident_ `":"` _Expression_  
> _TypedField_ : _Ident_ `":"` _Type_  
> _ExpressionList_ : _Expression_ [`","` _Expression_]*  
