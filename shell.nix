{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [
	gcc
	binutils
	gnumake
	rustup
	openssl
	
	sass
	nodejs
  ];

  RUST_BACKTRACE = 0;
}

