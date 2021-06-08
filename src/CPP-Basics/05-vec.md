# Vectors
In C++, a vector is a dynamic list of items, that can shrink and grow in size. 
It is created using `std::vector<type> name;` and it can only store values of the same type.

To use vectors, it is necessary to `#include` the vector library

```c++
#include <iostream>
#include <vector>
 
int main() {
  
  std::vector<int> grades(3);
  
  grades[0] = 90;
  grades[1] = 86;
  grades[2] = 98;
  
}
```

During the creation of a C++ vector, the data type of its elements must be specified. Once the vector is created, the type cannot be changed.

## Index
An index refers to an element’s position within an ordered list, like a vector or an array. The first element has an index of `0`.

A specific element in a vector or an array can be accessed using its index, like `name[index]`.

```c++
std::vector<int> grades = {65, 78, 90, 85}

std::cout << grades[2];
// Outputs: 90
```

## Vector Sizes   
The `.size()` function can be used to return the number of elements in a vector, like `name.size()`.

```c++
std::vector<std::string> employees;
 
employees.push_back("michael");
employees.push_back("jim");
employees.push_back("pam");
employees.push_back("dwight");
 
std::cout << employees.size();
// Prints: 4
```

## Push and Pop
The following functions can be used to add and remove an element in a vector:

- `.push_back()` to add an element to the “end” of a vector
- `.pop_back()` to remove an element from the “end” of a vector
```c++
std::vector<std::string> wishlist;
 
wishlist.push_back("GTX 3090");
wishlist.push_back("Ryzen 5 5600");
 
wishlist.pop_back();
 
std::cout << wishlist.size(); 
// Prints: 1
```