# confert

Turn config files into html code blocks.

```bash
Usage: confert <file> [--size N] [--output FILE]
```

## Example

Config file:

```
return {
  {
    "neovim/nvim-lspconfig",
    opts = {
      servers = {
        elixirls = false,
      },
    },
  },

  {
    "mason-org/mason-lspconfig.nvim",
    opts = {
      ensure_installed = {},
      automatic_installation = false,
    },
  },
}
```

Html:

```
<code><span>return {</span>
<span>&nbsp;{</span>
<span>&nbsp;&nbsp;"neovim/nvim-lspconfig",</span>
<span>&nbsp;&nbsp;opts = {</span>
<span>&nbsp;&nbsp;&nbsp;servers = {</span>
<span>&nbsp;&nbsp;&nbsp;&nbsp;elixirls = false,</span>
<span>&nbsp;&nbsp;&nbsp;},</span>
<span>&nbsp;&nbsp;},</span>
<span>&nbsp;},</span>

<span>&nbsp;{</span>
<span>&nbsp;&nbsp;"mason-org/mason-lspconfig.nvim",</span>
<span>&nbsp;&nbsp;opts = {</span>
<span>&nbsp;&nbsp;&nbsp;ensure_installed = {},</span>
<span>&nbsp;&nbsp;&nbsp;automatic_installation = false,</span>
<span>&nbsp;&nbsp;},</span>
<span>&nbsp;},</span>
<span>}</span></code>
```
