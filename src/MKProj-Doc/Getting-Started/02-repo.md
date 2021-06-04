# Working on a Repository
Before you start working on a repository, make sure you have the following installed: 

## Git 
For any work, git must be installed to commit and push code to GitHub 

```bash
# Debian Linux 
$ sudo apt install git-all
# Mac
git --version
```
Windows: Visit this [link](https://git-scm.com/download/win)

## Development Work

**Rust**: Visit [rustup.rs](http://rustup.rs) for the properly installation method for your system. 

Once you have Rust, you can install the following crates: 
```bash
# Markdown Book for documentations 
$ cargo install mdbook
# Crowbook to make Md -> html, epub and latex/pdf
$ cargo install crowbook
```

## Content Work
For content work, we recommend learning LaTex as it is a very important skill to learn personally. 
To install latex, do the following: 

```bash
# Debian Linux 
$ sudo apt-get install texlive-full
# MacOS
$ wget https://mirror.ctan.org/systems/mac/mactex/MacTeX.pkg | brew install MacTeX.pkg
```
Windows: Visit this [link](https://miktex.org/download/ctan/systems/win32/miktex/setup/windows-x64/basic-miktex-21.2-x64.exe)

Most Content work will usually reside with working on MarkDown related projects, and 99% of them use MDBook