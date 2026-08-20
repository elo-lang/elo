" Vim syntax file
" Language: Elo

if exists("b:current_syntax")
  finish
endif

autocmd BufNewFile,BufRead *.keymap setfiletype elo

" 1. Keywords & Statements
syn keyword eloStatement where import move share clone give macro it defer module ok fail some none self as break const continue else enum extern fn for if in let var match mut return ret sizeof struct while
syn keyword eloBoolean true false

" 2. Regex Matches (Identifiers, Constants, Numbers, Types)
syn match eloIdentifier "\<fn [a-z0-9_]\+"
syn match eloConstant "\<[A-Z][A-Z_0-9]\+\>"
syn match eloConstant "@library(system)\|@library\|@unsafe\|@raw\|@system\|@end\|\<value\>\|\<type\>"
syn match eloNumber "\<[0-9]\+\>"
syn match eloType "\<[A-Z]\+[a-zA-Z_0-9]*[a-z]\+[a-zA-Z_0-9]*\>"
syn match eloBuiltinType "\<\(bool\|str\|int\|uint\|float\|[iu]\(8\|16\|32\|64\)\|f32\|f64\|string\)\>"

" 3. Regions (Strings & Comments)
syn region eloString start='"' end='"' skip='\\.' contains=eloSpecialChar
syn region eloString start="'" end="'" skip='\\.' contains=eloSpecialChar
syn region eloCharacter start="`" end="`" skip='\\.' contains=eloSpecialChar
syn match eloSpecialChar contained "\\."

syn region eloComment start="//" end="$" contains=eloTodo
syn keyword eloTodo contained TODO XXX FIXME

" 4. Link Elo groups to standard Vim highlight groups
hi def link eloStatement Statement
hi def link eloBoolean Boolean
hi def link eloIdentifier Identifier
hi def link eloConstant Constant
hi def link eloNumber Number
hi def link eloType Type
hi def link eloBuiltinType Type
hi def link eloString String
hi def link eloCharacter Character
hi def link eloSpecialChar SpecialChar
hi def link eloComment Comment
hi def link eloTodo Todo

let b:current_syntax = "elo"
