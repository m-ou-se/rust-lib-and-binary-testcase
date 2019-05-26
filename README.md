Run `./test` for a demonstration.

Output:

```
   Compiling loadme v0.1.0 (/home/mara/dev/rust/libbin-test)
    Finished dev [unoptimized + debuginfo] target(s) in 0.28s

-> Loading and running foo() from the cdylib...

Hello, world!
Thread name: None

-> Loading and running foo() from the binary...

Hello, world!
./test: line 13: 15157 Segmentation fault      (core dumped) ./target/run target/debug/loadme
```
