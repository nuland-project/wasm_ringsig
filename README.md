# wasm-ringsig

idea is to compile [ringsig rust library] to wasm format using [wasm-bindgen] library

# For somehow below guide does not work for me and returns error:
"/home/misha/IdeaProjects/tmp/wasm_ringsig/node_modules/webpack-cli/bin/config-yargs.js:89
 				describe: optionsSchema.definitions.output.properties.path.description,
 				                                           ^

 TypeError: Cannot read property 'properties' of undefined
     at module.exports (/home/misha/IdeaProjects/tmp/wasm_ringsig/node_modules/webpack-cli/bin/config-yargs.js:89:48)
"

I managed to fix this error by copy-paste folder "node_modules" from another folder with workable project, but this folder must not be added to git

need to mention that `package.json` is same.

## how to run:
 * go through [wasm-bindgen tutorial] and install rust dependencies, create dist and build folders
 * in order to install npm dependencies run
 ```bash
 npm init
 ```

 * in order to build project
    ```bash
    npm run build
    ```
 * to run project locally:
    ```bash
     npm run serve
     ```
     and navigate to `localhost:8080`


# Issue with rust -> webassembly:
if we manage to run project, we can see that simple functions like
```src/lib.rs:get_str()```

```src/lib.rs:concat_strs_2()```

```src/lib.rs:collect_numbers()```
do work

and complex rust code like
```src/lib.rs:some_sign()``` (which just calculates sign with some hardcoded params)
does not work with error

`
wasm-0057bd12-1290:2 Uncaught (in promise) RuntimeError: unreachable
    at __rust_start_panic (wasm-function[1290]:1)
    at rust_panic (wasm-function[1289]:30)
    at std::panicking::rust_panic_with_hook::ha9e38c87c58c2caf (wasm-function[1286]:444)
    at std::panicking::continue_panic_fmt::h9205f39271e0037f (wasm-function[1285]:122)
    at rust_begin_unwind (wasm-function[1284]:3)
    at core::panicking::panic_fmt::hb924789bc7f51233 (wasm-function[1404]:70)
    at core::result::unwrap_failed::h32f56f6bb77c0c2b (wasm-function[633]:338)
    at _$LT$core..result..Result$LT$T$C$$u20$E$GT$$GT$::expect::h273b3c7f7b0f1b89 (wasm-function[632]:257)
    at fujisaki_ringsig::sig::sign::h958830c85277c9a6 (wasm-function[455]:2693)
    at react_rust_wasm::sign_rust::hcc2045f09b00e916 (wasm-function[263]:803)
`

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE))
 * MIT license ([LICENSE-MIT](LICENSE-MIT))

at your choice.




[ringsig rust library]: https://crates.io/crates/fujisaki_ringsig
[wasm-bindgen]: https://rustwasm.github.io/wasm-bindgen/
[wasm-bindgen tutorial]: https://rustwasm.github.io/wasm-bindgen/whirlwind-tour/basic-usage.html
