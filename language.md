```
run print println = {
    println "Hello, world!";
    print "And...";
    print "[finished]!";
    Ok
};

a = 1;
b = a;
helper x =
    add 2 x;
b = 2;

```


```
println "Hello, world!"
```

```
(ls (cwd)) |> map (\d -> [d, dirname d] >> \[x,y] `$x -> $y` >> println)
```

```
z for x in y
```
is the same as:
```
map z y
```

```
for i in (ls .) [i, dirname i]
|> \[orig, dir] -> `$orig -> $dir`
|> println
```

```
for i in (ls (cwd)) [i, dirname i] |> \[orig, dir] -> `$orig -> $dir` |> println
```

```
{   cdir =
        dirname
            (cwd)
            (os);
    print ls (dirname cdir);
    print `$(cwd) -> $cdir`;
    cdir
}
```

```
for i in (ls (cwd)) {
    println `We are in $(i)!`;
    println ("Great meeting you!\n";
          ++ "We are the business unit of acme.\n"
          ++ "Please provide your name: ");
    
}
```