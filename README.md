# BF Compiler
This is a ~barely~ fully functional compiler for the esoteric programming language Brainf\*ck.

## What is Brainf\*ck?
Brainf\*ck is an esolang invented by Urban Muller in 1993. Since then, it's become the most well known esolang, second only to JavaScript.
It consists of a total of nine elements:
```
+ - < > . , [ ]
```
... and your will to live.

Brainf\*ck is intentionally minimal, aggressively unreadable and the most undebuggable language to ever exist.
If that sounds just perfect for you, then you came to the right repo.

You can learn more about it [here](https://esolangs.org/wiki/Brainfuck)

## What is this?
This is a compiler made for this language. 
You can write your beautifully cursed code into a .bf file, and the compiler will compile it into an executable
with a guaranteed segfault in 3 seconds or less.

> [!NOTE] 
> This compiler is built specifically for x86\_64 linux, and will not run anywhere else.

## Installation
1. Download the tar.gz release and unzip it.
2. cd into the folder.
```sh  
# usually
$ cd ~/Downloads/bfc_release
```
3. Execute the installer script.
```sh 
$ ./installer.sh 
```
4. And that's it. You might need to restart your terminal.

## Usage
1. Create a .bf file and write your code in it.
2. To compile, use
```sh
$ bfc /path/to/your/file.bf
```
3. The compiler will create a
   - assembly file
   - object file
   - executable file
   - all with the same name as the .bf, in the same folder.
5. Execute your executable.
