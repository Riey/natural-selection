let
  pkgs = import <nixpkgs> { };
in
pkgs.mkShell {
  name = "rspkg-shell";

  buildInputs = with pkgs; [
    alsaLib
    libudev
    libxkbcommon
    vulkan-tools
    vulkan-headers
    vulkan-loader
    vulkan-validation-layers
  ];
  nativeBuildInputs = with pkgs; [
    pkgconfig
    rustc
    cargo
  ];
}
