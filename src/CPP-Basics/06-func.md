# Functions
A *function* is a set of statements that are executed together when the function is called. Every function has a name, which is used to call the respective function.

A C++ function has two parts:

- Function declaration
- Function definition
The declaration includes the function’s name, return type, and any parameters.

The definition is the actual body of the function which executes when a function is called. The body of a function is typically enclosed in curly braces.

A function can look like the example below:  
```c++
#include<iostream>
void hello(); //Function Declaration 

int main(){
    hello(); //Function call
    // Prints Hello!
}

void hello(){ //Function Definition
    std::cout << "Hello!";
}
```

## Parameters 
Function parameters are placeholders for values passed to the function. They act as variables inside a function.
They are placed in the parantheses in the function declaration, `int add(int a, int b)`. 

Consider the example below: 
```c++
#include<iostream>
void print_int(int i);

int main(){
    print_int(10); //Passes the value of 10 to the parameter i
    //Prints 10
}

void print_int(int i){
    std::cout << i;
}
```

## Return Values
A function that returns a value must have a `return` statement. The data type of the return value also must match the method’s declared return type.

On the other hand, a `void` function (one that does not return anything) does not require a return statement.

Consider the example below: 
```c++
int add(int a, int b);

int main(){
   int sum = add(6, 8); // sum = 14
    
}

void add(int a, int b){
    return a+b;
}

```

## Scope
The scope is the region of code that can access or view a given element:

- Variables defined in global scope are accessible throughout the program.
- Variables defined in a function have local scope and are only accessible inside the function.

```c++
#include <iostream>
 
void print();
 
int i = 10;       // global variable
 
int main() { 
  std::cout << i << "\n"; 
}
 
void print() { 
  int j = 0;      // local variable
  i = 20;
  std::cout << i << "\n"; 
  std::cout << j << "\n";
}
```