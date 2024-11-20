(module
  "module" @context) @item

(module_field_type
  "type" @context
  identifier: (_)? @name) @item

(module_field_global
  "global" @context
  identifier: (_)? @name) @item

(module_field_table
  "table" @context
  identifier: (_)? @name) @item

(module_field_memory
  "memory" @context
  identifier: (_)? @name) @item

(module_field_func
  "func" @context
  identifier: (_)? @name) @item

(module_field_elem
  "elem" @context
  identifier: (_)? @name) @item

(module_field_data
  "data" @context
  identifier: (_)? @name) @item
