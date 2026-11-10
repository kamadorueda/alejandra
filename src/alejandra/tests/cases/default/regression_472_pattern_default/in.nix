[
  ({nixpkgs ? import <nixpkgs> {}}: nixpkgs)
  ({ pkgs ? import <nixpkgs> {}, ... }: pkgs)
  ({ x ? 1 }@args: args)
  ({pkgs}: pkgs)
  ({pkgs, ...}: pkgs)
]
