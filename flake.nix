{
  description = "Solving advent of code with Rust and Nix";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, rust-overlay }:
    let
      nameValuePair = name: value: { inherit name value; };
      genAttrs = names: f: builtins.listToAttrs (map (n: nameValuePair n (f n)) names);
      allSystems = [ "x86_64-linux" "aarch64-linux" "x86_64-darwin" "aarch64-darwin" ];
      overlays = [ (import rust-overlay) ];
      forAllSystems = f: genAttrs allSystems (system: f {
        pkgs = import nixpkgs { inherit system overlays; };
      });
    in
    {
      packages = forAllSystems ({ pkgs }: {
        default = pkgs.rustPlatform.buildRustPackage {
          name = "advent-of-rust";
          src = ./.;
          cargoLock = {
            lockFile = ./Cargo.lock;
          };
        };
      });
      
      
      devShells = forAllSystems ({ pkgs }: {
        default = pkgs.mkShell {
          buildInputs = [
            pkgs.rust-analyzer
            pkgs.rust-bin.stable.latest.default
            pkgs.vscode-extensions.vadimcn.vscode-lldb
          ];
        };
      });
    };
}
