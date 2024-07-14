# earthlyls: earthly language server

A fast language server for [earthly].

![Screenshot of yage Earthfile in helix](screenshot.png)

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

## Text editor configuration

### Visual Studio Code

The `earthlyls` extension for Visual Studio Code is available in the [marketplace](https://marketplace.visualstudio.com/items?itemName=glehmann.earthlyls).

### neovim

`neovim` has a ready to use configuration for `earthlyls` in [nvim-lspconfig](https://github.com/neovim/nvim-lspconfig).

`earthlyls` can either be installed manually or with [mason.nvim](https://github.com/williamboman/mason.nvim), which
is configured to download the binary for your platform from [earthlyls’ releases](https://github.com/glehmann/earthlyls/releases).

### Zed

The [`earthfile` extension for Zed](https://github.com/glehmann/earthfile.zed) is available in the Zed extensions.
Open the command palette with `Cmd-Shif-P`, enter `zed: extensions` and install the `earthfile` extension.

### helix

Helix from the 24.07 branch comes preconfigured with earthlyls support. Just [install it](INSTALL.md) and enjoy!

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


## I like earthlyls. How do I say thanks?

Please [give earthlyls a star on GitHub](https://github.com/glehmann/earthlyls).

Contributions are very welcome and every bug report, support request, and feature request helps make earthlyls better.
Thank you :)

## License

`earthlyls` is distributed under the terms of the MIT license.

See [LICENSE](LICENSE) for details.

[earthly]:https://earthly.dev/

