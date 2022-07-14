# Pre-Commit integration

In order to use Alejandra with
[Pre-Commit](https://pre-commit.com/)
just create a file named `.pre-commit-config.yaml`
with contents:

```yaml
repos:
  - repo: https://github.com/kamadorueda/alejandra
    rev: 2.0.0
    # Choose either the 'alejandra' or 'alejandra-system' hook
    # depending on what pre-requisites you have:
    hooks:
      # Requires Nix to be previously installed in the system
      - id: alejandra

      # Requires Alejandra to be previously installed in the system
      - id: alejandra-system
```
