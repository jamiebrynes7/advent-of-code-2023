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

          rustPlatform = pkgs.makeRustPlatform {
            cargo = toolchain;
            rustc = toolchain;
          };

          aoc-cli = rustPlatform.buildRustPackage rec {
            pname = "aoc-cli";
            version = "90c86580efd50cc45f971563ecd555fcf8e689f1";

            src = pkgs.fetchFromGitHub {
              owner = "tobias-walle";
              repo = "advent-of-code-cli";
              rev = version;
              hash = "sha256-YozoYVXlBa7x0mBhs0Z2CImX47Yx5bltLzluM/UehdM=";
            };

            # Need to include Apple's build dependencies only if we are on a darwin system
            buildInputs = if pkgs.stdenv.isDarwin then
              with pkgs.darwin.apple_sdk; [
                frameworks.CoreFoundation
                frameworks.CoreServices
                frameworks.SystemConfiguration
              ]
            else
              [ ];

            cargoHash = "sha256-vSn24Stru0WjCBCkDd4oQnm65N2qTCcHVsX2nAZnV5Q=";

            doCheck = false;
          };

        in {
          default = devenv.lib.mkShell {
            inherit inputs pkgs;
            modules = [{
              packages = [ toolchain aoc-cli ];
              env.RUST_SRC_PATH = "${toolchain}/lib/rustlib/src/rust/library";

              scripts.load-day.exec = ''
                cargo new --vcs none --name day_$1 --bin days/day_$1
                pushd days/day_$1
                  aoc download -y 2023 -d $1
                popd
              '';
            }];
          };
        });
    };
}
