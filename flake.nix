{
#eyJhbGciOiJIUzI1NiJ9.eyJqdGkiOiI3MDc5ZDQzZC01N2E3LTQ1NzQtYWE2NS00Nzc5MjhkNDkwZTciLCJzY29wZXMiOiJjYWNoZSJ9.neEzdaB5TYRMIVTPidSPGW50cWGlkaNMlmXEiLaAN_c
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/80a3e9ca766a82fcec24648ab3a771d5dd8f9bf2";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
    nix-vscode-extensions.url = "github:nix-community/nix-vscode-extensions";
    argus.url = 
      "github:cognitive-engineering-lab/argus?rev=2cb5898ee5fb13621e73a49456cb2a9770ca2a82";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, nix-vscode-extensions, argus }:
    flake-utils.lib.eachSystem [ "x86_64-linux" "aarch64-linux" ] (system:
      let
        overlays = [ (import rust-overlay) nix-vscode-extensions.overlays.default ];
        pkgs = import nixpkgs { inherit system overlays; };

        arch-names = {
          "x86_64-linux" = "amd64";
          "aarch64-linux" = "aarch64";
        };

        supported-images = {
          "x86_64-linux" = {
            imageName = "ubuntu";
            imageDigest = "sha256:33d782143e3a76315de8570db1673fda6d5b17c854190b74e9e890d8e95c85cf";
            sha256 = "sha256-KGFXuZ6HCvbVMA7CIkn4HrmSq5RYaETO4ziEkWTQiK0=";
            finalImageTag = "22.04";
            finalImageName = "ubuntu";
          };

          "aarch64-linux" = {
            imageName = "ubuntu";
            imageDigest = "sha256:23fdb0648173966ac0b863e5b3d63032e99f44533c5b396e62f29770ca2c5210";
            sha256 = "sha256-XEa6epttG3nv7fL89dHELcGXtIDY+b6tF6F3w2iWg1Y=";
            finalImageTag = "22.04";
            finalImageName = "ubuntu";
          };
        };

        argus-original = argus.packages.${system};
        inherit (argus-original) argus-cli argus-book argus-ide argus-extension;
        toolchain = pkgs.rust-bin.fromRustupToolchainFile "${argus}/rust-toolchain.toml";

        host = "0.0.0.0";
        port = "8888";

        dockerEnv = with pkgs; [
          argus-cli
          codium-with-argus
          on-startup
          run-evaluation
          open-evaluation
          open-workspace
          open-tutorial
          julia-bin
          pkg-config
          coreutils
          binutils
          gnused
          cacert
          gcc
          bashInteractive
          alsa-lib.dev
          udev.dev
          toolchain
          nodejs_22
        ];

        run-evaluation = pkgs.writeScriptBin "run-evaluation" ''
          cd argus && ARGUS_DNF_PERF=  cargo test -p argus-cli && cd -

          node ${argus-ide}/packages/evaluation/dist/evaluation.cjs -h --rankBy=inertia &&
          node ${argus-ide}/packages/evaluation/dist/evaluation.cjs -h --rankBy=vars &&
          node ${argus-ide}/packages/evaluation/dist/evaluation.cjs -h --rankBy=depth &&

          mkdir -p evaluation/data/gen
          mv argus/crates/argus-cli/*.csv evaluation/data/gen/
          mv *.csv evaluation/data/gen/
          # NOTE the compiler data was hand-tuned an compared between
          # authors, but Julia will expect it to be present in the `gen` directory as well.
          cp evaluation/data/heuristic-precision\[rust\].csv evaluation/data/gen/
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
          cd -
        '';

        open-workspace = pkgs.writeScriptBin "open-workspace" ''
          mkdir -p ~/root
          codium --no-sandbox --user-data-dir=~/root argus/examples/hello-server/src/main.rs argus/examples/hello-server
        '';

        codium-with-argus = pkgs.vscode-with-extensions.override {
          vscode = pkgs.vscodium;
          vscodeExtensions = [
            pkgs.open-vsx-release.rust-lang.rust-analyzer
            argus-extension
          ];
        };

        study-source = pkgs.fetchFromGitHub {
          owner = "gavinleroy";
          repo = "argus-study";
          rev = "8fade9b499bf6268aae04626659149b3056a7948";
          hash = "sha256-fBJ61drnt3es2CXNdrDDMs9ogOpjaeEZXYO/Y3OAYZ0=";
        };

        evaluation-source = pkgs.fetchFromGitHub {
          owner = "gavinleroy";
          repo = "argus-eval";
          rev = "4eee678c6a3b4ec637505d11437ffc7ca80a696f";
          hash = "sha256-CtYpYQjDTXaxoopA9NP/UyM45H6jSIjAX2PrvfY9NMs=";
        };

        on-startup = pkgs.writeScriptBin "on-startup" ''
          #!/bin/bash
          cp ${argus-cli}/lib/bindings.ts argus/ide/packages/common/src/
          /bin/bash
        '';

        dockerImage = pkgs.dockerTools.buildLayeredImage {
          name = "gavinleroy/pldi25-argus-${arch-names.${system}}";
          tag = "latest";
          fromImage = pkgs.dockerTools.pullImage (supported-images.${system});

          contents = pkgs.buildEnv {
            name = "image-root";
            paths = dockerEnv;
          };

          extraCommands = ''
            mkdir -p argus
            mkdir -p argus-study
            mkdir -p evaluation
            cp -R ${argus}/* argus/
            cp -R ${study-source}/* argus-study/
            cp -R ${evaluation-source}/evaluation evaluation
          '';

          config = {
            Cmd = [ 
              "${pkgs.bashInteractive}/bin/bash" 
              "-c" 
              "${on-startup}/bin/on-startup"
            ];
            WorkingDir = "/";
            Env = with pkgs; [ 
              "DISPLAY=:0"
              "HOST=${host}"
              "PORT=${port}"
              "PATH=${lib.makeBinPath dockerEnv}:${argus}/scripts:/argus/scripts:/argus/ide/node_modules/.bin" 
              "LD_LIBRARY_PATH=${lib.makeLibraryPath dockerEnv}"
              "CARGO_TARGET_DIR=/tmp"
              "PKG_CONFIG_PATH=${udev.dev}/lib/pkgconfig:${alsa-lib.dev}/lib/pkgconfig"
              "PYTHON=${python3}"
              "LIBERTINE_PATH=${libertine}/share/fonts"
              "PLAYWRIGHT_BROWSERS_PATH=${playwright-driver.browsers}"
              "SSL_CERT_FILE=${cacert}/etc/ssl/certs/ca-bundle.crt"
            ];
            ExposedPorts."${port}/tcp" = {};
          };
        };
      in {
        packages.default = dockerImage;
      });
}
