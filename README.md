# Jail
## About
It's an simple compiled programming language written fully in Rust. Jail have very easy syntax, you can use it to write variouse applications, also is it memory safe. Jail have very easy to use documentation where you can find everything about jail syntax and other things.
 
## Contributing
If you want to contribute fork this repository and start writting some cool staff or do TODO's that are marked by this type of comment `// TODO: <what to todo>` so every developer know what to do

## Documentation
You can find our documentation in folder docs, language docs are in folder `docs/lang/` that's all. If you want to add something into our docs just contribute

## Example syntax
Example Mao program that will print out sum of 2 and 3 looks something like this:
```go
import "std.ja"

proc main() => int {
    println("Hello, World")
    return 0
}
```
More examples can be found in folder `/examples/`

## How to build it?
If you want to build mao interpreter and also c api lib on linux you can use bash file named `build.sh` this will output mao interpreter and c api in folder `output`
