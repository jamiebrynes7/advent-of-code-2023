{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-23.05";
    systems.url = "github:nix-systems/default";
    devenv.url = "github:cachix/devenv";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  nixConfig = {
    extra-trusted-public-keys =
      "devenv.cachix.org-1:w1cLUi8dv3hnoSPGAuibQv+f9TZLr6cv/Hm9XgU50cw=";
    extra-substituters = "https://devenv.cachix.org";
  };

  outputs = { self, nixpkgs, devenv, systems, rust-overlay, ... }@inputs:
    let forEachSystem = nixpkgs.lib.genAttrs (import systems);
    in {
      packages = forEachSystem (system: {
        devenv-up = self.devShells.${system}.default.config.procfileScript;
      });

      devShells = forEachSystem (system:
        let
          overlays = [ (import rust-overlay) ];
          pkgs = import nixpkgs { inherit system overlays; };

          toolchain = pkgs.rust-bin.stable.latest.default.override {
            extensions = [ "rust-analyzer" "rust-src" ];
          };
        in {
          default = devenv.lib.mkShell {
            inherit inputs pkgs;
            modules = [{
              packages = [ toolchain ];
              env.RUST_SRC_PATH = "${toolchain}/lib/rustlib/src/rust/library";
            }];
          };
        });
    };
}
