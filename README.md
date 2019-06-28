cbindgen-expand-bug
===================

This is a minimal example for reproducing Bug #347.

I ran into [#347] on a different repository. I now tried to reproduce it with a minimal example. The problem seems to have to do with locking files.

Even if you let the expansion happen in a different directory, there seems to be a locking deadlock.

```console
$ lslocks|grep cargo
cargo           10199  FLOCK       WRITE  0          0          0 /home/vmx/src/rust/cbindgen/hello/target/debug/.carg -lock
cargo           12301  FLOCK       WRITE* 0          0          0 /home/vmx/src/rust/cbindgen/hello/expanded/debug/.cargo-lock
cargo           10204  FLOCK       WRITE  0          0          0 /home/vmx/src/rust/cbindgen/hello/expanded/debug/.cargo-lock
```

Process 12301 tries to get the lock, but can't.

This is easily repoducible with this repo. Just run it like that:

```console
$ CARGO_EXPAND_TARGET_DIR=./expanded cargo build
…
   Compiling cbindgen-expand-bug v0.1.0 (/home/vmx/src/rust/cbindgen/cbindgen-expand-bug)
    Building [====================================================>  ] 101/103: cbindgen-expand-bug(build)
```

You can now start to watch the locks:

```console
$ watch 'lslocks|grep cargo
Every 2.0s: lslocks|grep cargo                                                           gene: Fri Jun 28 17:16:15 2019

cargo           29961  FLOCK       WRITE* 0          0          0 /tmp/cbindgen-expand-bug/expanded/debug/.cargo-lock
cargo           27746  FLOCK       WRITE  0          0          0 /tmp/cbindgen-expand-bug/expanded/debug/.cargo-lock
cargo           25472  FLOCK       WRITE  0          0          0 /tmp/cbindgen-expand-bug/target/debug/.cargo-lock
```

In the beginning you will only see a `WRITE` lock on the `target` directory. Next will be the lock on `expanded` and finally a second `WRITE` lock will appear on expanded.

[#347]: https://github.com/eqrion/cbindgen/issues/347


License
-------

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
