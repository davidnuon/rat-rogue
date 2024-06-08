{
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.05";

  inputs.flake-utils.url = "github:numtide/flake-utils";

  outputs = {
    self,
    nixpkgs,
    flake-utils,
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = nixpkgs.legacyPackages.${system};
      in {
        devShell = pkgs.mkShell {
          shellHook = ''
            export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${
              with pkgs;
                lib.makeLibraryPath [
                  libGL
                  xorg.libX11
                  xorg.libXi
                  xorg.libXcursor
                  xorg.libXrandr
                ]
            }"'';
          buildInputs = with pkgs; [
            cargo
            rustc
            cmake
            evcxr
          ];
        };
      }
    );
}
