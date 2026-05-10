# nix-build -E 'with import <nixpkgs> {}; callPackage ./paperpass-bin.nix {}'

# Guide:
# https://github.com/NixOS/nixpkgs/blob/master/doc/languages-frameworks/rust.section.md

{
  lib,
  fetchFromGitHub,
  rustPlatform,
  git
}:

rustPlatform.buildRustPackage (finalAttrs: {
  pname = "paperbackup";
  version = "1.2.0";

  # cargoLock.lockFile = ./Cargo.lock;

  src = fetchFromGitHub {
    owner = "Elephant9748";
    repo = "paperpass";
    # tag = "${finalAttrs.version}";
    rev = "refs/heads/main";
    hash = "sha256-W0bIsEnA/kmnJTEUeTpTDCEdbNCWawM1tQdGdijvJuY=";
  };

  # cargoPatches = [ ./0001-cargo-lock.patch ];

  # postPatch = ''
  #       if ! [ -f Cargo.lock ]; then
  #               cargo generate-lockfile
  #       fi
  #       # ln -sf ${./Cargo.lock} Cargo.lock
  #       ls -la
  # '';

  nativeBuildInputs = [ git ];

  cargoHash = "sha256-iSY2BmUQ/X9+Uu7G0uINFuhZB2ieU++shV30RSGKt94=";

  meta = {
    description = "paperpass-git password manager in terminal";
    homepage = "https://github.com/Elephant9748/paperpass";
    changelog = "https://github.com/Elephant9748/paperpass/releases/tag/${finalAttrs.version}";
    license = lib.licenses.mit;
    maintainers = with lib.maintainers; [
      rigel
    ];
    mainProgram = "${finalAttrs.pname}";
  };
})

