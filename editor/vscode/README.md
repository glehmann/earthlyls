# earthlyls: earthly language server

A fast language server for [earthly].

![Screenshot of yage Earthfile in Visual Studio Code](https://raw.githubusercontent.com/glehmann/earthlyls/0.5.0/editor/vscode/screenshot.png)

`earthlyls` supports the following LSP features:

* completion
* diagnostics
* document symbol
* goto declaration
* goto definition
* hover
* incremental document update
* references
* semantic tokens
* watch file changes
* workspace symbol

## Performance

How fast?

On a macbook air m1, `earthlyls` loads the 178 `Earthfile` in the `earthly` repository — approximately 10000 lines —
in 51.95ms. A simple "go to definition" runs under a millisecond. A "go to reference", which searchs in all the
`Earthfile` in the workspace, runs in 18.61ms.


## License

`earthlyls` is distributed under the terms of the MIT license.

See [LICENSE](LICENSE) for details.

[earthly]:https://earthly.dev/
