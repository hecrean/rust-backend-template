with import <nixpkgs> {};
mkShell {
  buildInputs = [
    postgresql,
		diesel-cli
  ];
}