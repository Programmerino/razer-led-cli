{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
  };

  outputs = { self, nixpkgs, flake-utils, naersk }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = nixpkgs.legacyPackages."${system}";
        naersk-lib = naersk.lib."${system}";
      in
        rec {
          packages.razer-led-cli = naersk-lib.buildPackage {
            pname = "razer-led-cli";
            root = ./.;
          };
          defaultPackage = packages.razer-led-cli;

          apps.razer-led-cli = flake-utils.lib.mkApp {
            drv = packages.razer-led-cli;
          };
          defaultApp = apps.razer-led-cli;

          devShell = pkgs.mkShell {
            nativeBuildInputs = with pkgs; [ rustc cargo ];
          };
        }
    );
}
