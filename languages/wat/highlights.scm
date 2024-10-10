(comment_block) @comment
(comment_line) @comment

[
  "block"
  "data"
  "declare"
  "elem"
  "else"
  "end"
  "export"
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
  "offset"
  "param"
  "result"
  "start"
  "table"
  "then"
  "type"
] @keyword

(int) @number
(float) @number

(op_nullary) @operator
(op_index) @operator
(op_index_opt) @operator
(op_index_opt_offset_opt_align_opt) @operator
(op_simd_offset_opt_align_opt) @operator
(op_const) @operator
(op_func_bind) @operator
(op_let) @operator
(op_select) @operator
(op_simd_const) @operator
(op_simd_lane) @operator
(op_table_copy) @operator
(op_table_init) @operator

[
  "(" ")"
] @punctuation.bracket

(string) @string

(value_type) @type.builtin

(identifier) @variable
(nat) @variable
