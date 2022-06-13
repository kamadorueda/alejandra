# Publishing the extension to open-vsx

> https://open-vsx.org/extension/kamadorueda/alejandra

```sh
$ vsce package
$ ovsx create-namespace kamadorueda -p "${TOKEN}"
$ ovsx publish -p "${TOKEN}" ./alejandra-*.vsix
```
