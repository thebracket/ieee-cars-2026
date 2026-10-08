# Safe Error Handling

Two bug classes account for a *staggering* share of security advisories:

1. **Null pointer dereference.** Tony Hoare called null his "billion-dollar mistake",
   and the CVE database agrees.
2. **Ignored return codes.** The function told you it failed. Nobody listened.

You've probably written this C code, or maintained it:

```c
FILE *f = fopen("config.json", "r");
char buffer[1024];
fgets(buffer, sizeof(buffer), f); // If the file didn't exist, f is NULL...
```

If `config.json` is missing, that's undefined behavior — a crash, or worse, on a
system where it isn't. And the *information that it failed was right there in the
return value*. Nothing in the language requires you to look at it. Linters can
nag; production code ignores them anyway.

Rust's answer to both bug classes is the **same feature**: a return type that
carries either the value or the failure — and a compiler that refuses to let you
touch the value without first proving you considered the failure.

> The code for this is in `code/ex03_errors`.

Let's meet the two types behind it.
