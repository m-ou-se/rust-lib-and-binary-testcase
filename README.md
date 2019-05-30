`loadme.rs` contains a function `foo()`, which will print `"Hello, world!"` on
one line, and the current thread name (`std::thread::current().name()`) on the
next.

When that file is compiled as a `cdylib` and we load and execute that function in a C program (using `dlopen`/`dlsym`), it works fine.
(The thread name is `None`, as there was no Rust start-up code to give the thread a name.)

When that file is compiled as a `binary` and run directly, it also works fine.
(The thread name is `Some("main")`.)

**However**, when we make `foo()` visible *from the binary* (with `-Wl,--export-dynamic`),
and we load and execute that function in a C program (using `dlopen`/`dlsym`),
it *segfaults*!

For a demonstration of this, run `./test`.

Output:

```
-> Loading and running foo() from the cdylib...

Hello, world!
Thread name: None

-> Loading and running foo() from the binary...

Hello, world!
./test: line 13: 15157 Segmentation fault      (core dumped) ./target/run target/debug/loadme
```

If we load `foo()` not into a C program, but into a Rust program, the following happens:

(Run `cargo run --example=run`.)

```
-> Loading and running foo() from the cdylib...

Hello, world!
Thread name: None

-> Loading and running foo() from the binary...

Hello, world!
Thread name: Some("main")
```

The `cdylib` version of `foo()` still doesn't see any thread name, but the `binary` version does.
I'm not sure why.

Interestingly, this happens on x86 and x86_64, but does not happen on my aarch64-based Android phone.
On that platform, nothing segfaults, and all four cases report a thread name of `None`.

**What is going on?**

The binary version clearly assumes some kind of initialisation is done, whereas the cdylib version doesn't.
But I can't find what causes that difference. Does rustc compile code differently?
Is the standard library doing something interesting?
Something with pthreads and dlopen interacting weirdly?

Please open an issue if you have any suggestions. :)
