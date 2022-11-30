# Prmatch (Pattern Random Match)

Random text generator using parsed patterns

### Usage example

Basic example

```
$ prmatch cvc -s sample-seed
- Seed: 37694
- Output: qeq
```

More advanced example, pick a word

```
$ prmatch {[%hello][%world]}
- Seed: 1669390355085
- Output: hello
```

Generate a word using only the letters a, b, c

```
$ prmatch [%abc]#:2:5
- Seed: 1669390468358
- Output: aacba
```

Useful example, create an user name

```
$ prmatch [%myname_]ddd
- Seed: 1669391244818
- Output: myname_973
```

Repeat a pattern with a known seed

```
$ prmatch [%myname_]ddd -c 1669391244818
- Seed: 1669391244818
- Output: myname_973
```

Generate a list of words with a known seed

```
$ prmatch [%myname_]ddd -c 1669391244818 -n 5 --not-pretty
myname_973
myname_867
myname_363
myname_515
myname_886
```