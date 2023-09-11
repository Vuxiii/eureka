# eureka
William og Valdedalders _vanvittige_ compiler i Rust :)

## Language Examples

For a simple variable declaration

### Variables
#### For a constant
```
let a: type = some_value;
```

#### For a variable that can change
```
var a: type = some_value;
```

### Types

#### Tuples

#### Arrays

```
let my_array: [some_int] <type>;

let nested: [][] <type>;

```

#### Structs


```
type MyStruct: Struct {
    field_a: <type>;
    field_b: <type>;
};

let my_instance: MyStruct;

my_instance = {
    field_a = 2;
};

let another_one: MyStruct = {
    ...
}
```

#### Enums

```
type MyEnum: enum {
    field_a;
    field_b: <type>;
};  

let my_instance: MyEnum::field_b = {

};
```


### Function declarations

```
fn fun: (x: type, y: type) -> return type {
  Body
}
```
### Closures
```
let f: (x: type, y: type) -> return type {
   Body
};

let abe = f;
```

### Function Calls
```
let a = f(2, 4);
```

### Comments

#### For a comment:
```
# This is a comment
```
#### For doc-string:
```
#!
```

#### Multiline comment
```
## 
This is
A
Multiline 
Comment
##
```
## Language Spec
