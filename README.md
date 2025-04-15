# Artifact for PLDI'25 paper: An Interactive Debugger for Rust Trait Errors

This artifact contains the codebase, data, and scripts required to reproduce every part of the paper. A rough overview, in order of the paper's sections for which we've provided an artifact to verify:

- Section 4: Implementation

  The code for the [VSCode extension `Argus`](https://marketplace.visualstudio.com/items?itemName=gavinleroy.argus) is provided in the `argus` directory. The below instructions include a step for compiling the CLI and running it on sample data, but see the README in that directory for instructions on how to install and use the Argus VSCode extension. (Users would install the extension from the VSCode marketplace, which is what we recommend for reviewers.)

- Section 5: Evaluation
  - 5.1 User Study
    We provide the tasks created for the user study in the `argus-study` directory.

  - **5.1.2 Results**: The results can be reproduced in evaluation/notebook.jl. Instructions for running it are below.
  - **5.2 Inertia Analysis:** The results can be reproduced in evaluation/notebook.jl. Instructions for running it are below.

## Supported Claims

The results in Section 5 for the user study are fully reproducible; the numbers shown in the Julia Notebook should match those in the paper.

> :warning: Note: running the performance analysis for the inertia analysis within docker will not produce exact numbers.

The full instructions are below, but to summarize, only two commands are needed to test the artifact end to end:

1. `run-evaluation`, to build the tool and run tests on the included example crates.
2. `open-evaluation`, to start a server to the evaluation notebook that contains all the data and code for the analysis.

Two additional commands are available: `open-tutorial` and `open-workspace`. The former serves our published tutorial to `localhost:8888`, and the latter opens a VSCodium instance with Argus pre-installed. These two aren't necessary, but are mentioned later in the instructions.

> All commands included in the Docker image are just bash scripts. If they feel like black boxes, run `which <CMD NAME>` and `cat` the contents of the file to inspect what they do.

## Getting Started

The artifact is packaged as a Docker image, so the only system requirement is [Docker](https://www.docker.com/). [Rust](https://www.rust-lang.org/tools/install) and [VSCode](https://code.visualstudio.com/) (or [VSCodium](https://vscodium.com/)) may also be necessary if you choose to evaluate the IDE extension on your local machine.

All source code, tutorials, and examples, and data analyses are packaged into the Docker image. We distribute two images: `aarch64` built for ARM platforms (e.g., an M1 Mac), and `amd64` built for x86 platforms (everything else). The images are available on Zenodo, in the GitHub [release assets](https://github.com/gavinleroy/pldi25-artifact/releases) or on DockerHub [(aarch64)](https://hub.docker.com/repository/docker/gavinleroy/pldi25-argus-aarch64/general)[(amd64)](https://hub.docker.com/repository/docker/gavinleroy/pldi25-argus-amd64/general).

> Note, Zenodo does not publish release assets in the public record. See this [GitHub issue](https://github.com/zenodo/zenodo/issues/1235).

Download the image appropriate for your computer, and then run the following, replacing `<ARCH>` with either `aarch64` or `amd64` as appropriate.

```bash
docker load < pldi25-argus-<ARCH>.tar.gz
```

Run the container with the following command, the name and exposed port will be important later:

```bash
docker run --name argus-image -p 8888:8888 -it gavinleroy/pldi25-argus-<ARCH>
```

## Tool Evaluation

To verify that the Argus CLI is properly installed and working, you can enter one of our example projects:

```bash
cd argus/examples/hello-server
```

then, if you run `cargo argus obligations` a large JSON blob will be printed to the console. Navigate back to the artifact root `cd /artifact` to continue.

This step will verify that the code artifacts *build* and *run*. The below command will compile and run the tool on the included example workspaces; these are the same workspaces used to gather data for Figure 11 (Section 5.2.2) in the paper. The data gathered in this step will be piped into the analysis notebook, which you will look at in the [next step](#analysis-evaluation).

> :warning: stderr will have lines that start with `MISSING` or `ERROR`, but if the process exits with status `0` then there were no actual errors --- it's just a chatty script.

Run the following:

```bash
run-evaluation
```

This script gathers local data, which is placed in the `evaluation/data/gen/` directory that is recognized by the Julia notebook in the next step.

## Analysis Evaluation

If the above worked, then you can compare the local data with the data used in the paper evaluation. The next command will download the Julia dependencies and start a server within the container. After launching, navigate to `localhost:8888` to view the full notebook.

> **Note, navigate to the URL provided in the command output. The screenshot below shows what this will look like**

Run the following command:

```bash
open-evaluation
```

All numbers and figures should be the same except the performance numbers gathered in the container, these will vary.

> :warning: We have seen Pluto struggle to render the notebook after all packages are installed. If it's been more than 20 minutes and the notebook hasn't loaded, simply close the browser tab and open it again.

![Screenshot 2025-03-18 at 23 44 30](https://github.com/user-attachments/assets/ee2d1dc7-7bb0-4bab-bda9-6eb04fabcb06)

## IDE Evaluation

We distribute Argus as a VSCode extension available on the [VSCode Marketplace](https://marketplace.visualstudio.com/items?itemName=gavinleroy.argus) and the [Open VSX Registry](https://open-vsx.org/extension/gavinleroy/argus). As of last week there have been over 1.6k downloads.

The extension can be evaluated locally, or from within the docker container, we **strongly** suggest evaluating the IDE extension on your local machine.

### IDE Evaluation (Local)

Exit the docker container if you haven't already. You'll need to copy one of the example projects from the docker container to your host machine. We recommend doing this by running the following:

```bash
docker cp argus-image:/artifact/argus/examples/hello-server /tmp/hello-server
```

The above copied the `hello-server` project to `/tmp` on your host machine. Change into this project and open VSCode by running:

```bash
code /tmp/hello-server
```

Search for "Argus" in the extensions panel, and click install --- the current version is `v0.1.15`. *If you find yourself using Nix or trying to install Argus from source, you're diverging from the normal installation procedure.*

Open the `src/main.rs` file and the extension should install. We have an [online tutorial](https://cel.cs.brown.edu/argus/) for debugging the `hello-server` example if you'd like to follow along.

> The tutorial is also available within the docker container. You'll have to launch the container again (see [Getting Started](#getting-started)), then run the command: `open-tutorial`. Navigate to <http://localhost:8888> to read the tutorial.

### IDE Evaluation (Within Docker)

The docker image already includes an instance of [VSCodium](https://vscodium.com/) with Argus pre-installed, however, **we do not recommend evaluating the IDE extension this way.**

You must first setup X11 forwarding on your host machine in order to see and interact with the GUI window. We have tested this with MacOS, but the process is neither straightforward nor equivalent on all machines. Again, we strongly suggest evaluating the extension on your local machine. Nevertheless, here are instructions for setting up X11 forwarding: [MacOS](https://gist.github.com/sorny/969fe55d85c9b0035b0109a31cbcb088), [Linux](https://www.baeldung.com/linux/docker-container-gui-applications), and [Windows](https://medium.com/@potatowagon/how-to-use-gui-apps-in-linux-docker-container-from-windows-host-485d3e1c64a3).

Restart the docker container, but this time you'll need to pass additional arguments to forward the display. (Below is an example for MacOS, but even if you have a Mac, this command *may* be different.)

```bash
docker run \
  --rm
  -e DISPLAY=docker.for.mac.host.internal:0 \
  -p 8888:8888 \
  -it gavinleroy/pldi25-argus-<ARCH>
```

Iff you've set everything up correctly, the following command will launch vscodium on the `hello-server` example with Argus ready to go:

```bash
open-workspace
```

You can follow along with our online tutorial to see Argus in action: [online tutorial](https://cel.cs.brown.edu/argus/).

> The tutorial is also available within the docker container. You'll have to launch the container again (see [Getting Started](#getting-started)), then run the command: `open-tutorial`. Navigate to <http://localhost:8888> to read the tutorial.
