# Introduction

## Program Structure 
Consider the following hello world program, `hello.cpp`: 

```c++
#include<iostream> 

int main(){
    std::cout << "Hello World" << std::endl; 
    return 0;
}

```
The program runs from top to bottom, line by line:

- The first line instructs the compiler to locate the file that contains a library called `iostream`. 
  - This library contains code that allows for I/O (input & output).
- The `main()` function houses all the instructions for the program.

## Basic Output 
Now let's talk more about the `std::cout` in our program above. This is used to display output to the user's 
command line or terminal. To use `std::cout`, you must use it following `<<` and a string or variable you wish 
to output. 

`std::cout << "The answer of the test is: " << answer << std::endl;`

**Note:** `std::endl` is used to end the line of the output. 


## Comments
Comments are useful to document code, temporary debugging and in C++, it supports two different 
type of comments, single line `//` and multi-line `/* */`. Comments are ignored by the compiler 
at compiler time, making them a very good way to organize your code. 

```c++ 
// This single line will be ignored

/* 
The first C++ program written by MKProjects 
was only available for Linux on Sanp!
All of this will be ignored !!!
*/ 
```

## Compile and Run
Since we have our program, `hello.cpp`, we may as well compile and run it. 
```bash
# First compile the program with g++ 
$ g++ hello.cpp

# Now run the binary to execute the program 
$ ls
a.out hello.cpp

$ ./a.out 
Hello World
```
