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
# This is a one dimensional array of type <type>
let my_array: [number_of_elements] <type>;

# This is a 'two'-dimensional array of type <type>
# This gives us rows * cols elements of type <type>
let nested: [rows][cols] <type>;


let nested: [cols][rows] <type>;

nested[x][y] = 2;

for row in rows {
    for col in cols {
        nested[row][col] = 0;
    }
}

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

### Statements

#### Control Statements

<<<<<<< HEAD

##### If

```
if some_condition {
    body
} else if another_condition {
    body
} else {
    body
}
```

##### While

```
while condition {
    <body>
}
```

##### Do

```
do {
    <body>
} until some_condition;
```

##### For

```
for (init_statement; some_guard; progress_statement) {
    <body>
}

for element in container {

}
```

### Expressions

#### Match

```
let result: int = match some_expression {

}
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
