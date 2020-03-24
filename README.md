# That Lang
A simple interpreted language developed with 0 external libraries or tools just 100% pure Rust from the lexer to the interpreter!
*This language was created just for fun and it's not intended to be used in the real world :)*

## Supported features
- [x] Compile time type safety
- [x] Primitive types
- [x] Loops
- [x] Mathematical operators
- [ ] Functions
- [ ] Structs

## Examples
Program to compute the first 20 fibonacci numbers
```go
int t1 := 1;
int t2 := 1;
for int i := 0; i < 20; i := i + 1 {
	int next := t1 + t2;
	t1 := t2;
	t2 := next;
	print t1;
}
```
Other code samples are in the "example" directory.
