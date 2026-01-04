(block_comment) @comment
(line_comment) @comment

[
  "array"
  "block"
  "catch"
  "catch_ref"
  "catch_all"
  "catch_all_ref"
  "data"
  "declare"
  "elem"
  "else"
  "end"
  "export"
  "field"
  "final"
  "func"
  "global"
  "if"
  "import"
  "item"
  "local"
  "loop"
  "memory"
  "module"
  "mut"
  "null"
  "offset"
  "pagesize"
  "param"
  "rec"
  "ref"
  "result"
  "shared"
  "start"
  "struct"
  "sub"
  "table"
  "tag"
  "then"
  "try_table"
  "type"
  "unshared"
] @keyword

(integer) @number
(float) @number

(instr_name) @operator

[
  "(" ")"
] @punctuation.bracket

(string) @string

(num_type) @type.builtin
(vec_type) @type.builtin

(identifier) @variable
(index) @variable

(module_field_func
  (identifier) @function)
(extern_type_func
  (identifier) @function)
