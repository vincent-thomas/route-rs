{
  description = "A Rust project development environment using Nix flakes";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-23.05";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }: flake-utils.lib.eachDefaultSystem (system: let
    pkgs = import nixpkgs {
      inherit system;
    };
  in {
    devShells.default = pkgs.mkShell {
      buildInputs = with pkgs; [
        rustc
        cargo
        bacon
        rust-analyzer
        clippy
      ];

      shellHook = ''
        export RUST_SRC_PATH="$(rustc --print sysroot)/lib/rustlib/src/rust/src"
      '';
    };
  });
}
