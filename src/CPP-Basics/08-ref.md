# References & Pointers
## Memory Address
In C++, the memory address is the location in the memory of an object. It can be accessed with the "address of" operator, `&`.

Given a variable `random_var` the memory address can be retrieved by printing out `&random_var`. It will return something like: `0x7ffd7caa5b54`.

## Pointers
In C++, a pointer variable stores the memory address of something else.  
It is created using the `*` sign.  
Example: `int* pointer = &gum; //Gets address of something like 0x3fed7c9a8b578`

## References 
In C++, a reference variable is an alias for another object. It is created using the `&` sign. Two things to note:

1. Anything done to the reference also happens to the original.
2. Aliases cannot be changed to alias something else.

Example: `int &a = b; //Now a shares the same address as b`

## Pass-By-Reference
In C++, pass-by-reference refers to passing parameters to a function by using references.  

It allows the ability to:  

- Modify the value of the function arguments.
- Avoid making copies of a variable/object for performance reasons.

```c++
void swap_num(int &i, int &j) {
 
  int temp = i;
  i = j;
  j = temp;
 
}
 
int main() {
 
  int a = 100;
  int b = 200;
 
  swap_num(a, b);
 
  std::cout << "A is " << a << "\n"; //Prints 200
  std::cout << "B is " << b << "\n"; //Prints 100
}
```

## const Reference
In C++, pass-by-reference with `const` can be used for a function where the parameter(s) won’t change inside the function.

This saves the computational cost of making a copy of the argument.
```c++
int triple(int const &i) {
 
  return i * 3;
 
}
```

## Dereference
In C++, a dereference reference operator, `*`, can be used to obtain the value pointed to by a pointer variable.
```c++
int gum = 3;
 
// * on left side is a pointer
int* pointer = &gum;
 
// * on right side is a dereference of that pointer
int dereference = *pointer;
```