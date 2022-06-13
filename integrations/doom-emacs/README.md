# Doom-Emacs integration

In order to configure Alejandra in
[Doom-emacs](https://github.com/hlissner/doom-emacs)
just use the following:

```lisp
(set-formatter! 'alejandra "alejandra --quiet" :modes '(nix-mode))
```
