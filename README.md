# Artifact for PLDI'25 paper: An Interactive Debugger for Rust Trait Errors

This artifact contains the codebase, data, and scripts required to reproduce every part of the paper. A rough overview, in order of the paper's sections for which we've provided an artifact to verify:

- Section 4: Implementation
  The code for the [VSCode extension `Argus`](https://marketplace.visualstudio.com/items?itemName=gavinleroy.argus) is provided in the `argus` directory. The below instructions include a step for compiling the CLI and running it on sample data, but see the README in that directory for instructions on how to install and use the Argus VS Code extension.

- Section 5: Evaluation
  - 5.1 User Study
    We provide the tasks created for the user study in the `argus-study` directory.

  - **5.1.2 Results**: The results can be reproduced in evaluation/notebook.jl. Instructions for running it are below.
  - **5.2 Inertia Analysis:** The results can be reproduced in evaluation/notebook.jl. Instructions for running it are below.

## Supported Claims

The results in Section 5 for the user study are fully reproducible; the numbers shown in the Julia Notebook should match those in the paper.

*Note, running the performance analysis for the inertia analysis on your local machine may not produce exact numbers. But they should follow a similar trend.*

## Getting Started

The artifact is packaged as a Docker image, so the only system requirement is Docker. First, load the `gavinleroy/pldi25-argus-<ARCH>` image from the Zenodo repository. We distribute two images: `aarch64` built for ARM platforms (e.g., an M1 Mac), and `amd64` built for x86 platforms (everything else). The images are also available on DockerHub[(aarch64)](https://hub.docker.com/repository/docker/gavinleroy/pldi25-argus-aarch64/general)[(amd64)](https://hub.docker.com/repository/docker/gavinleroy/pldi25-argus-amd64/general).

Download the image appropriate for your computer, and then run the following, replacing `<ARCH>` with either `aarch64` or `amd64` as appropriate:

```bash
docker load -i gavinleroy/pldi25-argus-<ARCH>.tar.gz
```

Run the container with the following command (it's important to expose port `8888`):

```bash
docker run -p 8888:8888 -ti gavinleroy/pldi25-argus-<ARCH>
```

The following step will verify that the code artifacts build and run. The following command will compile and run the tool on the included example workspaces, the same used for Figure 11 (Section 5.2.2) in the paper. The data gathered in this step will be reflected in the data analysis notebook in the following step.

> :warning: stderr will have many lines that start with `MISSING` or `ERROR`, but if the process exits with status `0` then there were no actual errors. It's just a chatty script.

Run the following:

```bash
run-evaluation
```

If the above worked, then you can compare the local data with the data used in the paper evaluation. The next command will download the Julia dependencies and start a server within the container. After launching, navigate to `localhost:8888` to view the full notebook.

> **NOTE, navigate to the URL provided in the command output. The screenshot below shows what this will look like**

Run the following command:

```bash
open-evaluation
```

All numbers and figures should be the same except the performance numbers gathered in the container, these will vary.

![Screenshot 2025-03-18 at 23 44 30](https://github.com/user-attachments/assets/ee2d1dc7-7bb0-4bab-bda9-6eb04fabcb06)
