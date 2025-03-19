{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
    depot-js.url = 
      "github:cognitive-engineering-lab/depot?rev=3676b134767aba6a951ed5fdaa9e037255921475";
    nix-vscode-extensions.url = "github:nix-community/nix-vscode-extensions";
    argus.url = 
      "github:cognitive-engineering-lab/argus?rev=b6ece7289e5a33fd8846e8828c6a5df7cfec98a1";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, nix-vscode-extensions, depot-js, argus }:
    flake-utils.lib.eachSystem [ "x86_64-linux" "aarch64-linux" ] (system:
      let
        overlays = [ (import rust-overlay) nix-vscode-extensions.overlays.default ];
        pkgs = import nixpkgs { inherit system overlays; };

        supported-images = {
          "x86_64-linux" = {
            imageName = "ubuntu";
            imageDigest = "sha256:e5a6aeef391a8a9bdaee3de6b28f393837c479d8217324a2340b64e45a81e0ef";
            sha256 = "sha256-Tl83usHws5SLvtB7GhjvPFEybbRkHFGcQeMwKZFbHtI=";
            finalImageTag = "20.04";
            finalImageName = "ubuntu";
          };

          "aarch64-linux" = {
            imageName = "ubuntu";
            imageDigest = "sha256:4489868cec4ea83f1e2c8e9f493ac957ec1451a63428dbec12af2894e6da4429";
            sha256 = "sha256-V54Rp/yS4VRC4KQb/rLXisk7963QlacM1t4x7NLIJ3M=";
            finalImageTag = "20.04";
            finalImageName = "ubuntu";
          };
        };

        inherit (argus.packages.${system}) argus-cli argus-ide argus-book;
        toolchain = pkgs.rust-bin.fromRustupToolchainFile ./argus/rust-toolchain.toml;

        host = "0.0.0.0";
        port = "8888";

        run-evaluation = pkgs.writeScriptBin "run-evaluation" ''
          mkdir -p ./evaluation/data/gen
          cd argus 
          ARGUS_DNF_PERF= cargo test -p argus-cli
          cargo make init-bindings
          cargo make eval-all
          mv *.csv ../evaluation/data/gen
        '';

        open-evaluation = pkgs.writeScriptBin "open-evaluation" ''
          ${pkgs.julia-bin}/bin/julia -e '
            println("Setting up Pluto environment...")

            using Pkg
            if !haskey(Pkg.installed(), "Pluto")
              println("Installing Pluto...")
              Pkg.add("Pluto")
            end

            using Pluto
            Pluto.run(notebook="./evaluation/notebook.jl"; host="${host}", port=${port})
          '
        '';

        open-tutorial = pkgs.writeScriptBin "open-tutorial" ''
          cd ${argus-book}
          ${pkgs.python3}/bin/python3 -m http.server ${port}
        '';

        open-workspace = pkgs.writeScriptBin "open-workspace" ''
          mkdir -p ~/root
          codium --no-sandbox --user-data-dir=~/root argus/examples/bevy/src/main.rs argus/examples/bevy
        '';

        codium-with-argus = pkgs.vscode-with-extensions.override {
          vscode = pkgs.vscodium;
          vscodeExtensions = [
            pkgs.open-vsx-release.rust-lang.rust-analyzer
            argus-ide
          ];
        };

        artifact-source = builtins.path {
          name = "local-source";
          path = ./.;
        };

        dockerEnv = with pkgs; [
          argus-cli
          codium-with-argus
          artifact-source

          open-evaluation
          open-workspace
          open-tutorial

          pkg-config
          coreutils
          cacert
          gnugrep

          julia-bin
          bashInteractive
          alsa-lib.dev
          udev.dev

          # CLI DEPS
          llvmPackages_latest.llvm
          llvmPackages_latest.lld
          toolchain
          guile
          guile-json
          cargo-make

          # IDE DEPS
          depot-js.packages.${system}.default
          nodejs_20
          pnpm_9
          biome
        ];

        dockerImage = pkgs.dockerTools.buildLayeredImage {
          name = "pldi25-argus";
          tag = "latest";
          fromImage = pkgs.dockerTools.pullImage (supported-images.${system});

          contents = pkgs.buildEnv {
            name = "image-root";
            paths = dockerEnv;
          };

          config = {
            Cmd = [ "${pkgs.bashInteractive}/bin/bash" ];
            WorkingDir = "/";
            Env = with pkgs; [ 
              "PATH=${builtins.concatStringsSep ":" (builtins.map (path: "${path}/bin") dockerEnv)}" 
              "DISPLAY=:0"
              "HOST=${host}"
              "PORT=${port}"
              "PYTHON=${python3}"
              "LIBERTINE_PATH=\"${libertine}/share/fonts\""
              "RUSTC_LINKER=\"${llvmPackages.clangUseLLVM}/bin/clang\""
              "PLAYWRIGHT_BROWSERS_PATH=\"${playwright-driver.browsers}\""
            ];
            ExposedPorts."${port}/tcp" = {};
          };
        };
      in {
        packages.default = dockerImage;

        devShell = with pkgs; mkShell {
          nativeBuildInputs = [ pkg-config ];
          buildInputs = dockerEnv ++ [
            open-evaluation
            open-workspace
            argus-cli
            codium-with-argus
          ];

          ARGUS_IMAGE="${dockerImage}";
          PYTHON = python3;
          LIBERTINE_PATH = "${libertine}/share/fonts";
          RUSTC_LINKER = "${llvmPackages.clangUseLLVM}/bin/clang";
          PLAYWRIGHT_BROWSERS_PATH="${playwright-driver.browsers}";
        };
      });
}
