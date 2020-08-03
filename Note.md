# CHIP-8 

## Memory Map:

> ---
> | **RAM** | `4kb 12-Bit address`
>
> ---
>>**`0xFFF`** (`4095`) End of Chip-8 RAM
>
>---
>
>>| `0x200` to `0xFFF`\
>>| Chip-8   \
>>| Program / Data \
>>| Space
>
>---
>
>>**`0x600`** (`1536`) Start of ETI 660 Chip-8 programs
>
>---
>> |\
>> |\
>> |
> ---
>
>> **`0x200`** (`512`) Start of most Chip-8 programs
>
> ---
>
>>| `0x000` to `0x1FF` \
>>| Reserved for  \
>>|  interpreter  
>
> ---
>
>>**`0x000`** (`0`) Start of Chip-8 RAM
> ---

## Registers

- 16 * 8-bit general purpose register `(V0 - VE)`
- 8-bit flag register `(VF)`
- 16-bit Index register for memory address `(I)`
- 16-bit Program Counter `(PC)`


## Opcode 
- Bitmask 
> | 4    3   |  2    1    | \
> | 1111 1111 | 1111 1111 | \
> | Opcode    | Byte      |