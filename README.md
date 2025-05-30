# nyan-efi

https://rust-osdev.github.io/uefi-rs/




## Build

```sh
cargo build
cargo build --release
```

```sh
cargo build --target x86_64-unknown-uefi
cargo build --target aarch64-unknown-uefi
```

## Debug

```sh
cargo run
```

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

## Make img file 

```sh
dd if=/dev/zero of=nyan.img bs=1M count=64
mkfs.vfat -F32 nyan.img
```

```sh
mkdir -p mnt
sudo mount nyan.img mnt
sudo cp -r esp/* mnt/
sudo umount mnt
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
``
```sh
qemu-system-x86_64 -enable-kvm \
    -drive if=pflash,format=raw,readonly=on,file=OVMF_CODE_4M.fd \
    -drive if=pflash,format=raw,readonly=on,file=OVMF_VARS_4M.fd \
    -drive format=raw,file=nyan.img
```