{
  inputs = {
    nixpkgs = {
      url = "github:nixos/nixpkgs/nixos-unstable";
    };
    flake-utils = {
      url = "github:numtide/flake-utils";
    };
    fenix = {
      url = "github:nix-community/fenix";
    };
  };
  outputs = { nixpkgs, flake-utils, fenix, ... }: flake-utils.lib.eachDefaultSystem (system:
    let
      pkgs = import nixpkgs {
        inherit system;
      };
      rust = (with fenix.packages.${system}; combine [
        stable.rustc
        stable.cargo
        stable.clippy-preview
        stable.rustfmt-preview
        stable.rust-std
        targets.wasm32-unknown-unknown.stable.rust-std
        stable.rust-src
        rust-analyzer
      ]);
    in rec {
      defaultApp = flake-utils.lib.mkApp {
        drv = defaultPackage;
      };
      defaultPackage = (pkgs.makeRustPlatform {
        rustc = rust;
        cargo = rust;
      }).buildRustPackage {
        pname = "tangram";
        version = "0.6.0-dev";
        src = ./.;
        doCheck = false;
        nativeBuildInputs = with pkgs; [
          clang_12
          lld_12
        ];
        cargoSha256 = "sha256-Pc1VyJC6RycHsz5TrLkyXE/jfbz6pdETrtzHkvzHK14=";
        cargoBuildFlags = [ "--bin" "tangram" ];
        CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER = "clang";
        CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_LINKER = "lld";
      };
      devShell = pkgs.mkShell {
        buildInputs = with pkgs; [
          cachix
          cargo-insta
          clang_12
          createrepo_c
          doxygen
          dpkg
          elixir
          go
          lld_12
          mold
          nodejs-16_x
          python39
          rpm
          ruby
          rust
          rust-cbindgen
          sequoia
          sqlite
          wasm-bindgen-cli
        ];
        CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER = toString ./. + "/scripts/clang";
        CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_LINKER = "lld";
      };
    }
  );
}
