# MKProjects Books

The MKProject Books project is aimed to produce free, opensource books for 
self learners, and serve as an entry point, or a condensed guide for the public. 
In the books project, all books are worked in their own repository, but then their 
`book` directory contents are then added to the Books repository. This allows all the 
books to be under the `book.mkproj.com` subdomain. 

In MKProjects Books, we offer two different formats of 
each book: 

- Web: MDBook (MarkDown Book)
- PDF: LaTex PDF Document

All Books use the `Basics_Template` repository, and this includes 
a `MDBook` project and `Tex` directory which includes the template 
used for all books. The Books project is a more Content than Development 
job, however it is still recommended to know the following: 

- LaTex
- MarkDown
- Git Basics

Each book repository carries a useful `Makefile`, and it's as easy as:

```bash
# To build and commit
$ make 
# To build just MarkDown
$ make mdbook
# To build just Latex 
$ make latex 
# To serve the markdown book 
$ make serve
```

To view the plan of MKProject Books, visit [here](http://mkproj.com/Book-Plan/book/index.html)