# rools

### **branche:**

- **main** : stable release branch
- **feature/<name>** : new features
- **bugfix/<name>** : bug fixes
- **release/<version>** : release preparation

### **commit:**

- Use imperative, present tense (e.g. "add" not "added")
- No capital letter at the beginning
- No period at the end

```
[feat/fix/docs/style/test]: [short description of the main effect]

[full description explaining why this change is made and what it fixes]
```

### **code styl:**

- Try using a non-confusing naming scheme for new functions (with verb) and variable names. 
- Use `snake_case` for functions, variables, structures and enums
- Use `UPPERCASE` for constants
- Use constants/enums for errors (`ERR_INVALID_PORT`)
- Use spaces only and never TABs, 2 spaces for each new opening brace
- No space between keyword and parenthesis: `while(1)` / `while (1)`
- Spaces around operators (except postfix and unary) : `age += 1; true = !false; size += -2 + 3 * (a + b);`
- Comments : use only `/* */`, never `//` and doxygen format recommended:

```rust
/**
 * @brief Brief description
 * @param param_name Parameter description
 * @return Return description
 */
```