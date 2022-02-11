# Hello World!

Let's try to write our first code in Jail!

First, we need The Jail Programming  Language installed
if you do not know how to do it, we recommend that you read the guide on [**Downloading Jail**](https://github.com/SolindekDev/Jail/blob/main/Documentation/Download/Download.md) 

After installing Jail, you need to create the **.jail** file
For example, **helloworld.jail**

Then copy the following code into
this file

```
 space main [argv] [int]
     puts "Hello, World"
     ret 0
 end
 ```


if you don't want to write so much code, it's enough:
```
puts "Hello, World"
```

Now you probably thought, ***What does each line of code do?***
First let's explain the longer version of the code because that is more important

```space main [argv] [int]``` is a declaration of a function called **main** which is always executed at the beginning of the program, so far don't worry about ```[argv]``` and ```[int]```
these will be explained in more detail in later pages of the documentation

```puts" Hello, World "```
prints the given text to the screen

```ret 0``` returns the number 0 when the interpreter reaches the last line of the function, so the program will run correctly

the word ``end`` is just the end of a function, don't worry about it for now
