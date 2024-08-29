{
  inputs = {
    nixpkgs = {
      url = "github:NixOS/nixpkgs/nixos-unstable";
    };
    yafas = {
      url = "github:UbiqueLambda/yafas";
    };
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      yafas,
      fenix,
      ...
    }: yafas.allSystems nixpkgs ({ pkgs, system }: {
      devShells.default = pkgs.mkShell.override {
        stdenv = pkgs.stdenvAdapters.useMoldLinker pkgs.clangStdenv;
      } {
        buildInputs = with pkgs; [
          (fenix.packages.${system}.complete.withComponents [
            "cargo"
            "clippy"
            "rustc"
            "rustfmt"
            "rust-src"
            "rust-analyzer"
          ])
          cmake
          lldb
          pkg-config
          cmake
          openssl
          xorg.libxcb
          libxkbcommon
          vulkan-tools
          vulkan-headers
          vulkan-loader
          # vulkan-validation-layers
          xorg.libX11
          xorg.libXcursor
          xorg.libXi
          xorg.libXrandr
          wayland
          glslang
          SDL2
          glib
          fontconfig
          gtk3
          pango
          alsa-lib
        ];
        LD_LIBRARY_PATH = with pkgs; pkgs.lib.makeLibraryPath [
          glslang
          vulkan-loader
          libxkbcommon
          wayland
          SDL2
          glib
          fontconfig
          gtk3
          pango
          alsa-lib
        ];
        RUSTFLAGS = "-Zthreads=8";
      };
    }
  );
}
