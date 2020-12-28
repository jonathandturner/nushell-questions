# Current nushell issues

I wanted to build a prototype of a tool built on top of a shell. I wanted to try
`nushell` given I figured it might be possible to borrow its parser more easily
than more traditional shells.

I'm attempting to start from a very simple function — supply a string and
receive a result.

A couple of issues to start:

- [collect_string](https://github.com/nushell/nushell/blob/0.24.1/crates/nu-cli/src/stream/input.rs#L50)
  seems to be the best way to collect the result of an executed pipeline. But it
  doesn't support any results that aren't strings. [Adding parsing for each
  `Primitive`](https://github.com/max-sixty/nushell/blob/ffcc615e33e0f08300fd96324887e9677858d102/crates/nu-cli/src/stream/input.rs#L83)
  works, but I imagine isn't the best way — should something like `.to_string()`
  be implemented on `Primitive`?
  - This raises an error.

    ```sh
    cargo run -- --line "= 1 + 3"
    ```

  - This does not raise an error:

    ```sh
    cargo run -- --line "= 1 + 3 | autoview"
    ```

- I'm unclear when values are returned by `run_block` and when they're streamed
  directly to stdout. One confusing example is that running

  ```sh
  cargo run -- --line "= 1 + 3 | autoview"
  ```

  runs through to this code:

  ```rust
  let stream = executor::block_on(fut).unwrap();
  dbg!(stream);
  ```

  ...but the value doesn't seem to be in the actual stream, it seems to go
  straight to stdout, here's the end of the result:

  ```
  [src/main.rs:60] "Collecting" = "Collecting"
  [src/main.rs:62] "Result OK" = "Result OK"
  [src/main.rs:64] "Finished collecting" = "Finished collecting"
  [src/main.rs:87] stream = ""
  4⏎                                
  ```
