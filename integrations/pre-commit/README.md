# Pre-Commit integration

In order to use Alejandra with
[Pre-Commit](https://pre-commit.com/)
just create a file named `.pre-commit-config.yaml`
with contents:

```yaml
repos:
  - repo: https://github.com/kamadorueda/alejandra
    rev: 4.0.0
    # Choose either the 'alejandra' or 'alejandra-system' hook
    # depending on what pre-requisites you have:
    hooks:
      # No prerequisites
      - id: alejandra

      # Requires Nix to be previously installed in the system
      - id: alejandra-nix

      # Requires Alejandra to be previously installed in the system
      - id: alejandra-system
```

To use the latest hook, run `pre-commit autoupdate --freeze --repo=https://github.com/kamadorueda/alejandra`.
