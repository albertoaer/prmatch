# Prmatch (Pattern Random Match)

Random text generator using parsed patterns

## How to use it?

Currently the program accepts two arguments, the pattern to be parsed and an optional seed,
if the seed is not provided, it will be use the current unix timestamp by default

```
$ prmatch pattern <seed>
```

### Usage example

Basic example

```
$ prmatch c-v-c sample-seed
- Seed: 37694
- Output: qeq
```

More advanced example, pick a word

```
$ prmatch.exe  "{(%hello)-(%world)}"
- Seed: 1669390355085
- Output: hello
```

Generate a word using only the letters a, b, c

```
$ prmatch.exe  "(%abc)#:2:5"
- Seed: 1669390468358
- Output: aacba
```

Useful example, create an user name

```
$ prmatch.exe  "(%myname_)-d-d-d"
- Seed: 1669391244818
- Output: myname_973
```