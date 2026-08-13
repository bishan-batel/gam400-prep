{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        pkgsCross = pkgs.pkgsCross.aarch64-multiplatform;

        libPath = with pkgs; [
          libffi
          spirv-tools
          vulkan-volk
          vulkan-tools
          vulkan-loader
          vulkan-headers
          vulkan-validation-layers
        ] ++ (if pkgs.stdenv.isLinux then (with pkgs; [
            wayland-protocols
            wayland.dev
            wayland
            libGL
            libxcb
            libX11
            libXrandr
            libXinerama
            libXcursor
            libXi
            libxkbcommon
            libglvnd
          ]) else []); 

        platformDeps = (if pkgs.stdenv.isDarwin then with pkgsCross; [ 
          libiconv 
        ] else with pkgsCross; [ 
            libdrm.dev 
            libdecor.dev
            mesa
          ]);

        rust-bin = rust-overlay.lib.mkRustBin { } pkgsCross.buildPackages;
      in
        {
        devShell = 
          pkgsCross.callPackage ( { mkShell, pkg-config, openssl, stdenv, }: mkShell {

            nativeBuildInputs = [
              (rust-bin.fromRustupToolchainFile ./toolchain.toml)
              pkg-config
              pkgs.cargo
              pkgs.rustc
            ] ++ platformDeps ++ libPath;

            buildInputs = with pkgs; [ 
              pkg-config 
              pkgsCross.stdenv.cc
              pkgs.cmake
            ];

            env = {
              # RUST_SRC_PATH = pkgs.rustPlatform.rustLibSrc;
              LD_LIBRARY_PATH = "${pkgs.lib.makeLibraryPath libPath}";
              PKG_CONFIG_ALLOW_CROSS = "1";


              VK_LAYER_PATH = "${pkgs.vulkan-validation-layers}/share/vulkan/explicit_layer.d";
              VULKAN_SDK = "${pkgs.vulkan-validation-layers}/share/vulkan/explicit_layer.d";

              CC_aarch64_unknown_linux_gnu = "${pkgsCross.stdenv.cc.targetPrefix}cc";
              CXX_aarch64_unknown_linux_gnu = "${pkgsCross.stdenv.cc.targetPrefix}c++";
              
              HOST_CC = "clang";

              # QEMU_FD = "${pkgs.qemu}/share/qemu/edk2-aarch64-code.fd";
              CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER = "${pkgsCross.stdenv.cc.targetPrefix}cc";
              CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_RUNNER = "qemu-aarch64";
            };
          }) {};
      }
    );
}
