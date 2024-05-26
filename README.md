# earthlyls: earthly language server

A fast language server for [earthly].

![Screenshot of yage Earthfile in helix](screenshot.png)

`earthlyls` supports the following LSP features:

* hover
* goto definition
* goto declaration
* references
* document symbol
* workspace symbol
* diagnostics
* semantic tokens
* incremental document update
* watch file changes

## Text editor configuration

### Visual Studio Code

The `earthlyls` extension for Visual Studio Code is available in the [marketplace](https://marketplace.visualstudio.com/items?itemName=glehmann.earthlyls).

### neovim

`neovim` has a ready to use configuration for `earthlyls` in [nvim-lspconfig](https://github.com/neovim/nvim-lspconfig).

`earthlyls` can either be installed manually or with [mason.nvim](https://github.com/williamboman/mason.nvim), which
is configured to download the binary for your platform from [earthlyls’ releases](https://github.com/glehmann/earthlyls/releases).

### helix

Helix from the `main` branch comes preconfigured with earthlyls support. Just [install it](INSTALL.md) and enjoy!

For older versions, add this in your `languages.toml`:

~~~toml
[language-server]
earthlyls = { command = "earthlyls" }

[[language]]
name = "earthfile"
scope = "source.earthfile"
injection-regex = "earthfile"
roots = ["Earthfile"]
file-types = [
  { glob = "Earthfile" },
]
comment-token = "#"
indent = { tab-width = 2, unit = "  " }
language-servers = ["earthlyls"]

[[grammar]]
name = "earthfile"
source = { git = "https://github.com/glehmann/tree-sitter-earthfile", rev = "2a6ab191f5f962562e495a818aa4e7f45f8a556a" }
~~~

Optionally run `hx --grammar build` to update your tree-sitter libraries and get the Earthfile syntax highlighting.

## Installation

See [INSTAll.md](INSTALL.md)

## Performance

How fast?

On a macbook air m1, `earthlyls` loads the 178 `Earthfile` in the `earthly` repository — approximately 10000 lines —
in 51.95ms. A simple "go to definition" runs under a millisecond. A "go to reference", which searchs in all the
`Earthfile` in the workspace, runs in 18.61ms.


## License

`earthlyls` is distributed under the terms of the MIT license.

See [LICENSE](LICENSE) for details.

[earthly]:https://earthly.dev/

