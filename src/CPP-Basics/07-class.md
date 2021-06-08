# Classes & Objects

A C++ class is an user-defined data type that may contain it's own unique methods, attributes, etc. 
To define your own class, use the `class` keyword along with a useful name for it. 

We will be defining our own class, and use it to describe the next sections: 

```c++
#include<strings>
class Summon{
    // Class attributes
    std::string name; 
    char type; 
    int tier; 
    std::string description; 

    // Constructor 
    Summon(std::string name, char type, int tier, std::string description); 

    //Private Methods:
    private:
    int what_dmg(Summon s){ //Finds dmg of a perticular summon
        if (s.type == 'T'){
            return (s.tier * 2 ) - 1; 
        } else (if s.type == 'S') {
            return s.tier * 2;
        } else {
            return 0;
        }
    }

    std::string what_type(Summon s){
        if (s.type == 'T'){
            return "Tech"; 
        } else if (s.type == 'S'){
            return "Striker";
        }

    }

    //Public Methods: 
    public: 
    void info(Summon s){
        std::cout  << "Name: "<< s.name << "\n" << "Type: " << what_type(s) << "\n" 
        << "Dmg: " << what_dmg(s) << "\n" << s.description << std::endl;
    }
};

```

## Class Members
A class is comprised of class members:

- Attributes, also known as member data, consist of information about an instance of the class.
- Methods, also known as member functions, are functions that can be used with an instance of the class.

This can be seen in our program above, things like `char type;` or `int tier;` are all class attributes. 
Our class methods were split into `private` and `public` methods *(talked in Access Control below)*, and are 
such things like `what_dmg` or `info`. 

### Objects
In C++, an object is an instance of a class that encapsulates data and functionality pertaining to that data.
This can be in our functions like `what_type(Summon s)` that uses the object `Summon s`. 

## Constructor
For a C++ class, a constructor is a special kind of method that enables control regarding how the objects of a class should be created. 
Different class constructors can be specified for the same class, but each constructor signature must be unique.

We have a constructor in our class, and is seen as `Summon(std::string name, char type, int tier, std::string description)`, and 
what it means is that to initialize a Summon instance, you must have the following. 

## Access Control 
C++ classes have access control operators that designate the scope of class members:

- `public`
- `private`  

`public` members are accessible everywhere; `private` members can only be accessed from within the same instance of the class or from friends classes.

You can see this when we don't want people to use our `what_dmg` or `what_type` functions, so we can avoid that 
by making those private, while info will be public, and use these private functions. 