# cursed-switch-macro
Slightly cursed declaritive macro for adding\* C style macros to Rust 
```c
switch(n) {
  case 1: something;
  case 2: something_else;
  default: probably_nothing;
}
```

---
# Notes
Yes, this is broken(specifically in the case of handling multiple cases without a default)  
\* within limitations of Rust's macro system and how I "want" to use it
