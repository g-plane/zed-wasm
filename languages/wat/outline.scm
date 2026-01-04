(module
  "module" @context) @item

(type_def
  "type" @context
  (identifier)? @name) @item

(module_field_global
  "global" @context
  (identifier)? @name) @item

(module_field_table
  "table" @context
  (identifier)? @name) @item

(module_field_memory
  "memory" @context
  (identifier)? @name) @item

(module_field_func
  "func" @context
  (identifier)? @name) @item

(module_field_tag
  "tag" @context
  (identifier)? @name) @item

(module_field_elem
  "elem" @context
  (identifier)? @name) @item
