with import <nixpkgs> {};

stdenv.mkDerivation {
  name = "kag";
  buildInputs = with pkgs; [
    pkgconfig
    openssl
  ];
}
