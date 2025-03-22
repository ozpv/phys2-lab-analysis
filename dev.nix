{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  packages = with pkgs; [
	rustc
	cargo
	clippy
	rustfmt
	lld
    fontconfig
    cargo-binutils
    cargo-leptos
    cargo-generate
    tailwindcss
    binaryen
	fontconfig
    postgresql
    sqlx-cli
    pkg-config
    twiggy
    openssl
	wasm-bindgen-cli
  ];
}
