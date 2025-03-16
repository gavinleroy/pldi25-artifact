{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    argus.url = "github:cognitive-engineering-lab/argus?rev=4de7f3191c3fc59820dad1c78b60d6c06895fdf3";
    nix-vscode-extensions.url = "github:nix-community/nix-vscode-extensions";
  };

  outputs = { self, nixpkgs, flake-utils, argus, nix-vscode-extensions }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ nix-vscode-extensions.overlays.default ];
        pkgs = import nixpkgs {
          inherit system overlays;
          crossSystem = {
            config = "x86_64-unknown-linux-gnu";
            system = "x86_64-linux";
          };
        };

        inherit (argus.packages.${system}) argus-cli argus-ide argus-book;

        open-evaluation = pkgs.writeScriptBin "open-evaluation" ''
          #!${pkgs.runtimeShell}

          ${pkgs.julia-bin}/bin/julia -e '
            println("Setting up Pluto environment...")

            using Pkg
            if !haskey(Pkg.installed(), "Pluto")
              println("Installing Pluto...")
              Pkg.add("Pluto")
            end

            using Pluto
            println("Opening notebook: $(notebook_path)")
            Pluto.run(notebook="./evaluation/notebook.jl")
          '
        '';

        open-tutorial = pkgs.writeScriptBin "open-tutorial" ''
          cd ${argus-book}
          ${pkgs.python3}/bin/python3 -m http.server
        '';

        open-workspace = pkgs.writeScriptBin "open-workspace" ''
          codium argus/examples/bevy/src/main.rs argus/examples/bevy
        '';

        codium-with-argus = pkgs.vscode-with-extensions.override {
          vscode = pkgs.vscodium;
          vscodeExtensions = [
            pkgs.open-vsx-release.rust-lang.rust-analyzer
            argus-ide
          ];
        };

        argus-source = pkgs.fetchFromGitHub {
          owner = "cognitive-engineering-lab";
          repo = "argus";
          # NOTE, should be the same as the `argus-pkgs` revision
          rev = "4de7f3191c3fc59820dad1c78b60d6c06895fdf3";
          sha256 = "sha256-NW+/h5wzn9afMoLt9ZKxnGn6Rszgamt4PhPFshSLCmw=";
        };

        local-source = builtins.path {
          name = "local-source";
          path = ./.;
        };

        artifact-source = pkgs.symlinkJoin {
          name = "artifact-source";
          paths = [ local-source argus-source ];
        };

        dockerImage = pkgs.dockerTools.buildLayeredImage {
          name = "pldi25-argus";
          tag = "latest";

          fromImage = pkgs.dockerTools.pullImage {
            imageName = "alpine";
            imageDigest = "sha256:1de5eb4a9a6735adb46b2c9c88674c0cfba3444dd4ac2341b3babf1261700529";
            sha256 = "sha256-6I6g8V5lwyPhiiw5qw9xgAjdwqBPhaYvuQgal3QOen0=";
            finalImageTag = "3.21.3";
            finalImageName = "alpine";
          };

          contents = pkgs.buildEnv {
            name = "image-root";
            paths = with pkgs; [
              # Custom derivations
              argus-cli
              codium-with-argus
              artifact-source

              # Commands
              open-evaluation
              open-workspace
              open-tutorial

              # From pkgs
              coreutils
              cacert
              julia-bin
              bashInteractive
            ];
          };

          config = {
            Cmd = [ "${pkgs.bashInteractive}/bin/bash" ];
            WorkingDir = "/";
            Env = [
              "PATH=${argus-cli}/bin:${codium-with-argus}/bin:${pkgs.bashInteractive}/bin:${pkgs.coreutils}/bin"
            ];
          };
        };
      in {
        packages.default = dockerImage;

        devShell = with pkgs; mkShell {
          nativeBuildInputs = [ pkg-config ];
          buildInputs = [
            open-evaluation
            open-workspace
            argus-cli
            codium-with-argus
          ] ++ lib.optionals stdenv.isDarwin [
            darwin.apple_sdk.frameworks.SystemConfiguration
          ] ++ lib.optionals stdenv.isLinux [
            alsa-lib.dev
            udev.dev
          ];

          shellHook = ''
            export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:$(rustc --print target-libdir)"
            export DYLD_LIBRARY_PATH="$DYLD_LIBRARY_PATH:$(rustc --print target-libdir)"
          '';

          IMAGE_PATH=dockerImage;

          PYTHON = python3;
          LIBERTINE_PATH = "${libertine}/share/fonts";
          RUSTC_LINKER = "${llvmPackages.clangUseLLVM}/bin/clang";
          PLAYWRIGHT_BROWSERS_PATH="${playwright-driver.browsers}";
        };
      });
}
