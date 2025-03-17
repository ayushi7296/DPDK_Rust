# DPDK_Rust

Using DPDK (Data Plane Development Kit) with Rust involves several steps, including setting up the DPDK environment, creating a Rust project, and writing Rust code to interact with DPDK. Below is a step-by-step guide to help you get started:
1. Set Up DPDK
First, you need to install and set up DPDK on your system.

Download DPDK:

Visit the DPDK website and download the latest stable release.

2. Extract and Build DPDK:
``` tar xf dpdk-<version>.tar.xz
    cd dpdk-<version>
    meson build
    cd build
    ninja
    sudo ninja install
```

3. Set Up Environment Variables:
Set the RTE_SDK and RTE_TARGET environment variables.
```
    export RTE_SDK=/path/to/dpdk
    export RTE_TARGET=x86_64-native-linuxapp-gcc
```
4. Hugepages Setup:

DPDK requires hugepages to be configured.
```
    sudo sh -c "echo 1024 > /sys/kernel/mm/hugepages/hugepages-2048kB/nr_hugepages"
    sudo mkdir -p /mnt/huge
    sudo mount -t hugetlbfs nodev /mnt/huge
```


