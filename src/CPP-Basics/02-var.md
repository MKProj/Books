# Variables

A variable refers to a storage location in the computer’s memory that one can set aside to save, retrieve, and manipulate data.

To initialize a variable in C++, you must first declare it's data type (will be discussed next), a name for the variable 
and assigned a value with the assignment operator `=`. 

Ex. `int var = 10; // data_type name = value; `

Variables can also be declared uninitialized, this means it doesn't have a value yet, and this is done by not adding an 
assignment value. 

Ex. `int var;`

## Data Types 
In C++, there are 4 primitive data types, these are integers (`int`), double floating point (`double`), 
characters (`char`), and boolean (`bool`). As well as a common data type used along the primitive type is strings(`std::string`). 

### Integers
The integer data type is a- non-decimal number that can be positive or negative. An integer variable 
is declared with the `int` keyword, and keep in mind it can not be manipulated along the double data type. 
An integer typically requires 4 bytes of memory space and ranges from -2³¹ to 2³¹.  
Ex. `int foo = 89`

### Doubles
The double data type is a decimal point number requiring 8 bytes of memory, and is declared using 
the `double` keyword.  
Ex. `double foo = 0.78`

### Characters
The character data type is a single character that is wrapped within single quotes `' '`. They typically 
require 1 byte of memory, and is declared by the `char` keyword. 
Ex. `char letter = 'A'`

### Strings
Strings are an array of characters and are wrapped within double quotes `" "`. To declare a string 
you will need to use `std::string`.  
Ex. `std::string word = "hello";`

### Boolean 
A boolean data type is a value that is either `true` or `false`, and is declared using the `bool` keyword 
Ex. `bool condition = true`

## Arithmetic Operations 
C++ supports different types of arithmetic operators that can perform common mathematical operations:

- `+` addition
- `-` subtraction
- `*` multiplication
- `/` division
- `%` modulo (yields the remainder)

## User Input
`std::cin` which stands for “character input”, reads user input from the keyboard.

Here, the user can enter a number, press `enter`, and that number will get stored in `tip`.
```c++
int tip = 0;
 
std::cout << "Enter amount: ";
std::cin >> tip;
```
