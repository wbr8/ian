{
  description = "Interactive AQA assembly language iNterpreter";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs = {
    self,
    nixpkgs,
  }: let
    pkgs = nixpkgs.legacyPackages."x86_64-linux";

    buildLibraries = [
    ];
  in {
    devShells."x86_64-linux".default = pkgs.mkShell {
      buildInputs =
        [
          pkgs.cargo
          pkgs.rustc
          pkgs.rustfmt
          pkgs.clippy
          pkgs.rust-analyzer
        ]
        ++ buildLibraries;

      nativeBuildInputs = [pkgs.pkg-config];

      env.RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
      env.LD_LIBRARY_PATH = "${pkgs.lib.makeLibraryPath buildLibraries}";
    };
  };
}
