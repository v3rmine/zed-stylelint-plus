# Zed Stylelint-plus Language Server Extension

## Disclaimer

Why fork [florian-sanders/zed-stylelint](https://github.com/florian-sanders/zed-stylelint) ?

~~Well I could not make it work on LESS files, and I knew that [neovim/nvim-lspconfig](https://github.com/neovim/nvim-lspconfig) used [bmatcuk/stylelint-lsp](https://github.com/bmatcuk/stylelint-lsp) to support stylelint so I changed the source of the lsp and it seems to work at least for the diagnostics.~~

**Just use the original repository it works with LESS now!**

## How to configure?

### General LSP settings

Settings and configuration tweaks are explained in details in the [bmatcuk/stylelint-lsp README](https://github.com/bmatcuk/stylelint-lsp/blob/master/README.md).

In your global or local settings, enable the language server by adding a `stylelint-plus` section in `lsp` section.

Settings can be passed to the LSP server by adding a `settings` section inside `stylelint-plus`.

For instance:
```JSONC
// settings.json
{
  "lsp": {
    "stylelint-plus": {
      "settings": {
        /* these are the default settings, you shouldn't need to set most of them, only add them as needed */
        // automatically apply fixes in response to format requests
        autoFixOnFormat: false, // Doesnt work
        // automatically apply fixes on save
        autoFixOnSave: false, // Doesnt work
        // stylelint config to use
        config: null,
        // path to stylelint config file
        configFile: null,
        // if false, disable linting and auto-formatting
        enable: true,
        // lint on save
        validateOnSave: true,
        // lint after changes
        validateOnType: true
      }
    }
  }
}
```

## Acknowledgment

This extension code is forked from [florian-sanders/zed-stylelint](https://github.com/florian-sanders/zed-stylelint).

As said above, the whole language server code comes from [bmatcuk/stylelint-lsp](https://github.com/bmatcuk/stylelint-lsp) so all the credits go to them really!
