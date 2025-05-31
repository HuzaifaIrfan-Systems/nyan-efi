<br />

<div align="center">
  <h1>NYAN EFI</h1>
  <p><h3 align="center">UEFI Boot to Nyan Cat 🚀</h3></p>
</div>

https://www.youtube.com/watch?v=cHK-yvoZUPw
•
https://rust-osdev.github.io/uefi-rs/
•
https://github.com/diekmann/uefi_nyan_80x25

<hr>



## Build

```sh
cargo build --target x86_64-unknown-uefi --release
cargo build --target aarch64-unknown-uefi --release
```

## Debug

```sh
cargo run --release
```

## Setup BOOT DIR

```sh
mkdir -p esp/EFI/BOOT
cp target/x86_64-unknown-uefi/release/nyan.efi esp/EFI/BOOT/BOOTX64.EFI
cp target/aarch64-unknown-uefi/release/nyan.efi esp/EFI/BOOT/BOOTAA64.EFI
```

## Setup QEMU

```sh
sudo apt-get install qemu ovmf
cp /usr/share/OVMF/OVMF_CODE_4M.fd .
cp /usr/share/OVMF/OVMF_VARS_4M.fd .
cp /usr/share/qemu-efi-aarch64/QEMU_EFI.fd .
```

## Run on QEMU

```sh
qemu-system-x86_64 -enable-kvm \
    -drive if=pflash,format=raw,readonly=on,file=OVMF_CODE_4M.fd \
    -drive if=pflash,format=raw,readonly=on,file=OVMF_VARS_4M.fd \
    -drive format=raw,file=fat:rw:esp
```

```sh
qemu-system-aarch64 \
  -machine virt \
  -cpu cortex-a57 \
  -accel tcg \
  -bios QEMU_EFI.fd \
  -device virtio-gpu-pci \
  -drive if=none,format=raw,file=fat:rw:esp,id=hd0 \
  -device virtio-blk,drive=hd0 \
  -serial stdio
```

## Copy EFI to EFI Partition

```sh
sudo mkdir /boot/efi/EFI/nyan/
sudo cp esp/EFI/BOOT/BOOTX64.EFI /boot/efi/EFI/nyan/BOOTX64.EFI
```

## Setup Entry in UEFI Firmware

```sh
sudo efibootmgr --create --disk /dev/sda --part 1 --label "nyan" --loader \\EFI\\nyan\\BOOTX64.EFI 
```

## Setup Entry in GRUB Boot Loader

```script
insmod part_gpt
insmod fat
search --no-floppy --file --set=root /EFI/nyan/BOOTX64.EFI
chainloader /EFI/nyan/BOOTX64.EFI
```


```sh
sudo update-grub
```

## Make Boot img file 

### Create a 64MB empty image

```sh
dd if=/dev/zero of=nyan.img bs=1M count=64
```

### Create a partition table and EFI partition using parted

```sh
parted nyan.img --script -- \
    mklabel gpt \
    mkpart ESP fat32 1MiB 100% \
    set 1 boot on \
    set 1 esp on
```

### Setup loop device with partitions

```sh
LOOP=$(sudo losetup --find --show --partscan nyan.img)
```

### Format the first partition as FAT32

```sh
sudo mkfs.vfat -F32 ${LOOP}p1
```

### Mount, copy files, unmount

```sh
mkdir -p mnt
sudo mount ${LOOP}p1 mnt
sudo cp -r esp/* mnt/
sudo umount mnt
```

### Detach the loop device

```sh
sudo losetup -d $LOOP
```


## Run img file 

```sh
qemu-system-x86_64 -enable-kvm \
    -drive if=pflash,format=raw,readonly=on,file=OVMF_CODE_4M.fd \
    -drive if=pflash,format=raw,readonly=on,file=OVMF_VARS_4M.fd \
    -drive format=raw,file=nyan.img
```


## 🤝🏻 &nbsp;Connect with Me

<p align="center">
<a href="https://www.huzaifairfan.com"><img src="https://img.shields.io/badge/-huzaifairfan.com-1aa260?style=flat&logo=Google-Chrome&logoColor=white"/></a>
<a href="https://www.linkedin.com/in/huzaifairfan/"><img src="https://img.shields.io/badge/-Huzaifa%20Irfan-0072b1?style=flat&logo=Linkedin&logoColor=white"/></a>
<a href="https://github.com/HuzaifaIrfan/"><img src="https://img.shields.io/badge/-Huzaifa%20Irfan-4078c0?style=flat&logo=Github&logoColor=white"/></a>
<a href="mailto:contact@huzaifairfan.com"><img src="https://img.shields.io/badge/-contact@huzaifairfan.com-c71610?style=flat&logo=Gmail&logoColor=white"/></a>
<a href="https://www.instagram.com/huzaifairfan2001/"><img src="https://img.shields.io/badge/-@huzaifairfan2001-cd486b?style=flat&logo=Instagram&logoColor=white"/></a>
<a href="https://www.facebook.com/huzaifairfan2001/"><img src="https://img.shields.io/badge/-@huzaifairfan2001-4267B2?style=flat&logo=Facebook&logoColor=white"/></a>
</p>

## License

Licensed under the MIT License, Copyright 2025 Huzaifa Irfan. [LICENSE](LICENSE)