# earthlyls: earthly language server

A fast language server for [earthly].

![Screenshot of yage Earthfile in helix](screenshot.png)

## Installation

### From sources

Just run:

~~~sh
cargo install --path .
~~~

## Text editor configuration

### helix

Helix from the `main` branch comes preconfigured with earthlyls support. Just enjoy!

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

## License

`earthlyls` is distributed under the terms of the MIT license.

See [LICENSE](LICENSE) for details.

[earthly]:https://earthly.dev/

