# Doom Emacs integration

In order to configure Alejandra in
[Doom Emacs](https://github.com/hlissner/doom-emacs)
just use the following:

```lisp
(after! nix-mode
  (set-formatter! 'alejandra '("alejandra" "--quiet") :modes '(nix-mode)))
```

If you've enabled formatting via LSP in Nix,
you might also need to add the following:

```lisp
(setq-hook! 'nix-mode-hook +format-with-lsp nil)
```
