# (Statically) T(yped) A(iving's) (scri)PT(ing) language aka TAPT (wtf lmao)

Just read Crafting Interpreters and this is the result.
I hope I don't abandon this project after a day.

## Syntax

```rust
record SimpleHuman(string, int, bool);

struct Human {
    name: string,
    age: int,
    is_dead: bool
}

const me = new Human {
    name: "Me",
    age: 23415,
    is_dead: true
};

println(me.name);

// Error! `me` is immutable variable
me = new Human {
    name: "Me",
    age: 23415,
    is_dead: true
};

// All ok, you can just do shadowing.
let me = new Human {
    name: "Me",
    age: 23415,
    is_dead: true
};

if true {
    // Error! Expected `Human` struct, found `SimpleHuman` record.
    me = new SimpleHuman("NotMe", -20, false);

    const me = new SimpleHuman("NotMe", -20, false);

    println(me.0);
}

func check_reality_stability(): string {
    const value = 50;
    
    match value {
        50 => "reality is stable",
        value => "REALITY IS UNSTABLE",
    }
}

println(check_reality_stability())
```

## Goals

- [ ] Modules
- [ ] Corelib
- [ ] Closures?
- [ ] Async support
- [ ] Something like interfaces or traits
- [ ] Loops
- [ ] Iterators
- [ ] Enums
- [ ] Generics
- [ ] Optimizations :fire:
