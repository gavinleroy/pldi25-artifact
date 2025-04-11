# Artifact for PLDI'25 paper: An Interactive Debugger for Rust Trait Errors

This artifact contains the codebase, data, and scripts required to reproduce every part of the paper. A rough overview, in order of the paper's sections for which we've provided an artifact to verify:

- Section 4: Implementation

  The code for the [VSCode extension `Argus`](https://marketplace.visualstudio.com/items?itemName=gavinleroy.argus) is provided in the `argus` directory. The below instructions include a step for compiling the CLI and running it on sample data, but see the README in that directory for instructions on how to install and use the Argus VSCode extension.

- Section 5: Evaluation
  - 5.1 User Study
    We provide the tasks created for the user study in the `argus-study` directory.

  - **5.1.2 Results**: The results can be reproduced in evaluation/notebook.jl. Instructions for running it are below.
  - **5.2 Inertia Analysis:** The results can be reproduced in evaluation/notebook.jl. Instructions for running it are below.

## Supported Claims

The results in Section 5 for the user study are fully reproducible; the numbers shown in the Julia Notebook should match those in the paper.

> :warning: Running the performance analysis for the inertia analysis within docker will not produce exact numbers.

## Getting Started

The artifact is packaged as a Docker image, so the only system requirement is [Docker](https://www.docker.com/). [Rust](https://www.rust-lang.org/tools/install) and [VSCode](https://code.visualstudio.com/) (or [VSCodium](https://vscodium.com/)) may also be necessary if you choose to evaluate the IDE extension on your local machine.

All source code, tutorials, and examples, and data analyses are packaged into the Docker image. First, load the `gavinleroy/pldi25-argus-<ARCH>` image from the Zenodo repository (or from the latest [GitHub release](https://github.com/gavinleroy/pldi25-artifact/releases)).

We distribute two images: `aarch64` built for ARM platforms (e.g., an M1 Mac), and `amd64` built for x86 platforms (everything else). The images are also available on DockerHub [(aarch64)](https://hub.docker.com/repository/docker/gavinleroy/pldi25-argus-aarch64/general)[(amd64)](https://hub.docker.com/repository/docker/gavinleroy/pldi25-argus-amd64/general). Download the image appropriate for your computer, and then run the following, replacing `<ARCH>` with either `aarch64` or `amd64` as appropriate.

```bash
docker load -i gavinleroy/pldi25-argus-<ARCH>.tar.gz
```

Run the container with the following command, the name and exposed port will be important later:

```bash
docker run --name argus-image -p 8888:8888 -it gavinleroy/pldi25-argus-<ARCH>
```

## Tool Evaluation

This step will verify that the code artifacts build and run. The below command will compile and run the tool on the included example workspaces; these are the same workspaces used to gather data for Figure 11 (Section 5.2.2) in the paper. The data gathered in this step will be piped into the analysis notebook, which you will look at in the [next step](#analysis-evaluation).

> :warning: stderr will have many lines that start with `MISSING` or `ERROR`, but if the process exits with status `0` then there were no actual errors --- it's just a chatty script.

Run the following:

```bash
run-evaluation
```

## Analysis Evaluation

If the above worked, then you can compare the local data with the data used in the paper evaluation. The next command will download the Julia dependencies and start a server within the container. After launching, navigate to `localhost:8888` to view the full notebook.

> **NOTE, navigate to the URL provided in the command output. The screenshot below shows what this will look like**

Run the following command:

```bash
open-evaluation
```

All numbers and figures should be the same except the performance numbers gathered in the container, these will vary.

![Screenshot 2025-03-18 at 23 44 30](https://github.com/user-attachments/assets/ee2d1dc7-7bb0-4bab-bda9-6eb04fabcb06)

## IDE Evaluation

We distribute Argus as a VSCode extension available on the [VSCode Marketplace](https://marketplace.visualstudio.com/items?itemName=gavinleroy.argus) and the [Open VSX Registry](https://open-vsx.org/extension/gavinleroy/argus). As of last week there have been over 1.6k downloads.

The extension can be evaluated locally, or from within the docker container, we **strongly** suggest evaluating the IDE extension on your local machine.

### IDE Evaluation (Local)

Exit the docker container if you haven't already. You'll need to copy one of the example projects from the docker container to your host machine. We recommend doing this by running the following:

```bash
docker cp argus-image:/argus/examples/hello-server /tmp/
```

The above copied the `hello-server` project to `/tmp` on your host machine. Change into this project and open VSCode by running:

```bash
cd /tmp && code .
```

Search for "Argus" in the extensions panel, and click install --- the current version is `v0.1.15`.

Open the `src/main.rs` file and the extension should install. We have an [online tutorial](https://cel.cs.brown.edu/argus/) for debugging the `hello-server` example if you'd like to follow along.

> The tutorial is also available within the docker container. You'd have to launch the container again (see [Getting Started](#getting-started)), then run the command: `open-tutorial`. Navigate to [`localhost:8888`](https://localhost:8888) to read the tutorial.

### IDE Evaluation (Within Docker)

The docker image already includes an instance of [VSCodium](https://vscodium.com/) with Argus pre-installed, however, **we do not recommend evaluating the IDE extension this way.**

You must first setup X11 forwarding on your host machine in order to see and interact with the GUI window. We have tested this with MacOS, but the process is neither straightforward nor equivalent on all machines. Again, we strongly suggest evaluating the extension on your local machine. Nevertheless, here are instructions for setting up X11 forwarding: [MacOS](https://gist.github.com/sorny/969fe55d85c9b0035b0109a31cbcb088), [Linux](https://www.baeldung.com/linux/docker-container-gui-applications), and [Windows](https://medium.com/@potatowagon/how-to-use-gui-apps-in-linux-docker-container-from-windows-host-485d3e1c64a3).

Restart the docker container, but this time you'll need to pass additional arguments to forward the display. (Below is an example for MacOS, but even if you have a Mac, this command *may* be different.)

```bash
docker run \
  --name argus-image \
  -e DISPLAY=docker.for.mac.host.internal:0 \
  -p 8888:8888 \
  -it gavinleroy/pldi25-argus-<ARCH>
```

Iff you've set everything up correctly, the following command will launch vscodium on the `hello-server` example with Argus ready to go:

```bash
open-workspace
```
