{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [
	gcc
	binutils
	gnumake

	rustup
	openssl
	pkgconfig
	
	nodejs
  ];

  RUST_BACKTRACE = 0;
}

