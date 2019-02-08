:
# generate binary and upload to MCU
ELF=blink-stm32f051
cargo size --bin $ELF -- -A
cargo objcopy --bin $ELF -- -Obinary /tmp/$ELF.bin
st-flash write /tmp/$ELF.bin 0x8000000
