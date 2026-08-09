{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  nativeBuildInputs = with pkgs; [
    pkg-config
  ];

  buildInputs = with pkgs; [
    dbus
    dbus.dev
    openssl
    cargo
    rustc
  ];

  shellHook = ''
    export PKG_CONFIG_PATH="${pkgs.dbus.dev}/lib/pkgconfig:$PKG_CONFIG_PATH"
  '';
}
