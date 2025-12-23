{
  inputs = {
    nixpkgs = {
      url = "github:nixos/nixpkgs/nixos-unstable";
    };
    flake-utils = {
      url = "github:numtide/flake-utils";
    };
  };
  outputs = { nixpkgs, flake-utils, self, ... }: flake-utils.lib.eachDefaultSystem (system:
    let
      pkgs = import nixpkgs {
        inherit system;
      };
    in rec {
      packages.default = pkgs.stdenv.mkDerivation {
          pname = "cpp";
          version = "1.0.0";
          src = self;
        };
      devShell = pkgs.mkShell {
        buildInputs = with pkgs; [
            cargo
            rustc
            rust-analyzer
            rustlings
            rustfmt
            clippy
        ];
      };
    }
  );
}

