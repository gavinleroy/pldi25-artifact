{
#eyJhbGciOiJIUzI1NiJ9.eyJqdGkiOiI3MDc5ZDQzZC01N2E3LTQ1NzQtYWE2NS00Nzc5MjhkNDkwZTciLCJzY29wZXMiOiJjYWNoZSJ9.neEzdaB5TYRMIVTPidSPGW50cWGlkaNMlmXEiLaAN_c
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/80a3e9ca766a82fcec24648ab3a771d5dd8f9bf2";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
    nix-vscode-extensions.url = "github:nix-community/nix-vscode-extensions";
    argus.url = 
      "github:cognitive-engineering-lab/argus?rev=b8a38d64dd1b4d2758389aee52f008ead3a15c71";
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

        argus-original = argus.packages.${system};
        inherit (argus-original) argus-cli argus-book argus-ide argus-extension;
        toolchain = pkgs.rust-bin.fromRustupToolchainFile "${argus}/rust-toolchain.toml";

        codium-with-argus = pkgs.vscode-with-extensions.override {
          vscode = pkgs.vscodium;
          vscodeExtensions = [
            pkgs.open-vsx-release.rust-lang.rust-analyzer
            argus-extension
          ];
        };

        host = "0.0.0.0";
        port = "8888";

        dockerEnv = [
          argus-cli
          codium-with-argus
          on-startup
          run-evaluation
          open-evaluation
          open-workspace
          open-tutorial
          toolchain

          pkgs.chromium
          pkgs.julia-bin
          pkgs.gcc
          pkgs.pkg-config
          pkgs.coreutils
          pkgs.cacert
          pkgs.bashInteractive
          pkgs.alsa-lib.dev
          pkgs.udev.dev
          pkgs.http-server
          pkgs.nodejs_22
        ];

        app-dir-name = "artifact";
        app-dir = "/${app-dir-name}";

        run-evaluation = pkgs.writeScriptBin "run-evaluation" ''
          cd ${app-dir}/argus && ARGUS_DNF_PERF= RUST_LOG=off cargo test -p argus-cli && cd -

          node ${argus-ide}/packages/evaluation/dist/evaluation.cjs -h --rankBy=inertia &&
          node ${argus-ide}/packages/evaluation/dist/evaluation.cjs -h --rankBy=vars &&
          node ${argus-ide}/packages/evaluation/dist/evaluation.cjs -h --rankBy=depth &&

          mkdir -p ${app-dir}/evaluation/data/gen
          mv ${app-dir}/argus/crates/argus-cli/*.csv ${app-dir}/evaluation/data/gen/
          mv ${app-dir}/*.csv ${app-dir}/evaluation/data/gen/
          # NOTE the compiler data was hand-tuned an compared between
          # authors, but Julia will expect it to be present in the `gen` directory as well.
          cp ${app-dir}/evaluation/data/heuristic-precision\[rust\].csv ${app-dir}/evaluation/data/gen/
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
          ${pkgs.http-server}/bin/http-server ${argus-book} -p ${port}
        '';

        open-workspace = pkgs.writeScriptBin "open-workspace" ''
          mkdir -p ${app-dir}/.root
          ${codium-with-argus}/bin/codium --no-sandbox --user-data-dir=${app-dir}/.root argus/examples/hello-server/src/main.rs argus/examples/hello-server
        '';

        chromium-wrapper = pkgs.writeScriptBin "nohd-wrap" '' 
          modified_args=()
          for arg in "$@"; do
              if [ "$arg" == "--headless=old" ]; then
                  modified_args+=("--headless=new")
              else
                  modified_args+=("$arg")
              fi
          done
          echo "Wrapper: Executing '$ORIGINAL_BINARY' with args: ''${modified_args[@]}" >&2
          exec ${pkgs.chromium}/bin/chromium "''${modified_args[@]}"
        '';

        # NOTE the chromium-1134 is very specific to the version of playwright being used
        # but we do this to avoid installing all of playwright, which includes firefox and webkit.
        browsers-dir = "${app-dir}/.playwright-browsers";
        chromium-linux-dir = "${browsers-dir}/chromium-1134/chrome-linux";
        on-startup = pkgs.writeScriptBin "on-startup" ''
          cp ${argus-cli}/lib/bindings.ts ${app-dir}/argus/ide/packages/common/src/
          mkdir -p ${chromium-linux-dir}
          ln -s ${chromium-wrapper}/bin/nohd-wrap ${chromium-linux-dir}/chromium
          ln -s ${chromium-wrapper}/bin/nohd-wrap ${chromium-linux-dir}/chrome
          ln -s ${chromium-wrapper}/bin/nohd-wrap ${chromium-linux-dir}/chromium-browser

          cd ${app-dir}
          "${pkgs.bashInteractive}/bin/bash"
        '';

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

        dockerImage = pkgs.dockerTools.buildLayeredImage {
          name = "gavinleroy/pldi25-argus-${arch-names.${system}}";
          tag = "latest";
          #fromImage = pkgs.dockerTools.pullImage (supported-images.${system});

          contents = pkgs.buildEnv {
            name = "image-root";
            paths = dockerEnv;
          };

          extraCommands = ''
            mkdir -p ${app-dir-name}
            mkdir -p ${app-dir-name}/argus
            mkdir -p ${app-dir-name}/argus-study
            mkdir -p ${app-dir-name}/evaluation
            cp -R ${argus}/* ${app-dir-name}/argus/
            cp -R ${study-source}/* ${app-dir-name}/argus-study/
            cp -R ${evaluation-source}/evaluation ${app-dir-name}/
          '';

          config = {
            Cmd = [ 
              "${pkgs.bashInteractive}/bin/bash" 
              "-c" 
              "${on-startup}/bin/on-startup"
            ];
            WorkingDir = "/";
            Env = [ 
              "DISPLAY=:0"
              "HOST=${host}"
              "PORT=${port}"
              "CARGO_TARGET_DIR=/tmp"
              "RUSTFLAGS=-L /lib"
              "LD_LIBRARY_PATH=/lib"
              "PKG_CONFIG_PATH=${pkgs.udev.dev}/lib/pkgconfig:${pkgs.alsa-lib.dev}/lib/pkgconfig"
              "LIBERTINE_PATH=${pkgs.libertine}/share/fonts"
              "PLAYWRIGHT_BROWSERS_PATH=${browsers-dir}"
              #"PLAYWRIGHT_BROWSERS_PATH=${pkgs.playwright-driver.browsers}"
              "SSL_CERT_FILE=${pkgs.cacert}/etc/ssl/certs/ca-bundle.crt"
            ];
            ExposedPorts."${port}/tcp" = {};
          };
        };
      in {
        packages.default = dockerImage;
      });
}
