# Artifact for PLDI'25 paper: An Interactive Debugger for Rust Trait Errors

This artifact contains the codebase, data, and scripts required to reproduce every part of the paper. A rough overview, in order of the paper's sections for which we've provided an artifact to verify:

- Section 4: Implementation
  The code for the [VSCode extension `Argus`](https://marketplace.visualstudio.com/items?itemName=gavinleroy.argus) is provided in the `argus` directory. See the README in that directory for instructions on how to install and use Argus.

- Section 5: Evaluation
  - 5.1 User Study
    We provide the tasks created for the user study in the `argus-study` directory.

  - **5.1.2 Results**: The results can be reproduced in evaluation/notebook.jl. Instructions for running it are below.
  - **5.2 Inertia Analysis:** The results can be reproduced in evaluation/notebook.jl. Instructions for running it are below.

## Supported Claims

The results in Section 5 for the user study are fully reproducible; the numbers shown in the Julia Notebook should match those in the paper. 

*Note, running the performance analysis for the inertia analysis on your local machine may not produce exact numbers. But they should follow a similar trend.*

# Getting Started

The artifact is packaged as a Docker image, so the only system requirement is Docker. First, load the `gavinleroy/pldi25-argus-<ARCH>` image (available on DockerHub[(aarch64)](https://hub.docker.com/repository/docker/gavinleroy/pldi25-argus-aarch64/general)[(amd64)](https://hub.docker.com/repository/docker/gavinleroy/pldi25-argus-amd64/general)). We distribute two images: `aarch64` built for ARM platforms (e.g., an M1 Mac), and `amd64` built for x86 platforms (everything else). Download the image appropriate for your computer, and then run the following, replacing `<ARCH>` with either `aarch64` or `amd64` as appropriate:

```bash
docker load -i gavinleroy/pldi25-argus-<ARCH>.tar.gz
```

Run the container with the following command (it's important to expose port `8888`):

```bash
docker run -p 8888:8888 -ti gavinleroy/pldi25-argus-<ARCH>
```

To verify that the code artifacts are working, first you can check that Argus (the name of the compiler extension) works correctly. For this task, run the following in the container:

```bash
run-evaluation
```

> :warning: stderr will have many lines that start with `MISSING` or `ERROR`, but if the process exits with status `0` then there were no actual errors. It's just a chatty script.

This script will run the Argus tool within the docker container, gathering the performance and accuracy numbers used in Figure 11 (Section 5.2.2).

After generating the data on your local machine, you can view the full evaluation by running the following command:

```bash
open-evaluation
```

The above will start a Julia server running within Docker, you will need to navigate to `localhost:8888` to view the notebook. **NOTE, navigate to the URL provided in the output of the above command. (See below image)** All numbers and figures should be the same except the performance numbers, which will vary.
