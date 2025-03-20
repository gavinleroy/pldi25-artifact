{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/80a3e9ca766a82fcec24648ab3a771d5dd8f9bf2";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
    depot-js.url = 
      "github:cognitive-engineering-lab/depot?rev=3676b134767aba6a951ed5fdaa9e037255921475";
    nix-vscode-extensions.url = "github:nix-community/nix-vscode-extensions";
    argus.url = 
      "github:cognitive-engineering-lab/argus?rev=6c98f73052bca7cb154e781519ac39c4e8f1f9b4";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, nix-vscode-extensions, depot-js, argus }:
    flake-utils.lib.eachSystem [ "x86_64-linux" "aarch64-linux" ] (system:
      let
        overlays = [ (import rust-overlay) nix-vscode-extensions.overlays.default ];
        pkgs = import nixpkgs { inherit system overlays; };

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

        inherit (argus.packages.${system}) argus-cli argus-ide argus-book;
        toolchain = pkgs.rust-bin.fromRustupToolchainFile "${argus}/rust-toolchain.toml";

        host = "0.0.0.0";
        port = "8888";

        run-evaluation = pkgs.writeScriptBin "run-evaluation" ''
          cd argus &&
          ARGUS_DNF_PERF= cargo test -p argus-cli &&
	  cargo make build &&
	  node ide/packages/evaluation/dist/evaluation.cjs -h --rankBy=inertia &&
	  node ide/packages/evaluation/dist/evaluation.cjs -h --rankBy=vars &&
	  node ide/packages/evaluation/dist/evaluation.cjs -h --rankBy=depth &&
          mkdir -p ../evaluation/data/gen
	  mv crates/argus-cli/*.csv ../evaluation/data/gen/
	  mv *.csv ../evaluation/data/gen/
	  # NOTE, compiler data is (partially) hand-tuned, so we copy it
	  cp ../evaluation/data/heuristic-precision\[rust\].csv ../evaluation/data/gen/
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

        evaluation-source = builtins.path {
          name = "evaluation-source";
          path = ./evaluation;
        };

        dockerEnv = with pkgs; [
          argus-cli
          codium-with-argus

          on-startup
          run-evaluation
          open-evaluation
          open-workspace
          open-tutorial

          pkg-config
          coreutils
          binutils
	  cacert
          bashInteractive
          gnused

          # Used in my own debugging
	  findutils
	  iconv
	  file
          gnugrep
	  strace
          neovim

          julia-bin
          alsa-lib.dev
          udev.dev

          # CLI DEPS
          gcc
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

        on-startup = pkgs.writeScriptBin "on-startup" ''
          #!/bin/bash
          cp ${argus-cli}/lib/bindings.ts argus/ide/packages/common/src/
          ln -sf ${pkgs.glibc}/lib/ld-linux-aarch64.so.1 /lib/ld-linux-aarch64.so.1
	  /bin/bash
	'';

        dockerImage = pkgs.dockerTools.buildLayeredImage {
          name = "gavinleroy/pldi25-argus-${builtins.elemAt (builtins.split "-" system) 0}";
          tag = "latest";
          fromImage = pkgs.dockerTools.pullImage (supported-images.${system});

          contents = pkgs.buildEnv {
            name = "image-root";
            paths = dockerEnv;
          };

          extraCommands = ''
            mkdir -p argus
            mkdir -p evaluation
            cp -R ${argus}/* argus/
            cp -R ${evaluation-source}/* evaluation/
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
