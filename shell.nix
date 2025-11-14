{ pkgs ? import <nixpkgs> {} }:

let e = 
pkgs.buildFHSEnv {
    name = "lux-packages";
    targetPkgs = ps: with ps; [ 
    rustc
    cargo
    clippy
    rustfmt
    rust-analyzer
  ];
};
in
e.env

