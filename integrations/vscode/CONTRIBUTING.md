# Publishing the extension to open-vsx

> https://open-vsx.org/extension/kamadorueda/alejandra

```sh
$ yarn install
$ yarn vsce package
$ ovsx create-namespace kamadorueda -p "${TOKEN}"
$ ovsx publish -p "${TOKEN}" ./alejandra-*.vsix
```
