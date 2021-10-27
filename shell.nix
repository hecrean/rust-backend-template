with import <nixpkgs> {};
mkShell {
  buildInputs = [
	postgresql
	diesel-cli
	llvm
	cmake
	apacheKafka
	nodePackages.npm
  ];
}