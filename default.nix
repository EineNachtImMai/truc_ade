{pkgs ? import <nixpkgs> {}}:
pkgs.rustPlatform.buildRustPackage rec {
  pname = "ade";
  version = "0.1";
  nativeBuildInputs = with pkgs; [pkg-config];
  buildInputs = with pkgs; [openssl];
  cargoLock.lockFile = ./Cargo.lock;
  src = pkgs.lib.cleanSource ./.;
}
