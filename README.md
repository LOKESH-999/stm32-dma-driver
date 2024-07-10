# VERIFYING DMA TRANSFER BY LED roulette


<p align="center">
<img src="https://i.imgur.com/0k1r2Lc.gif">
</p>

I have built this upon cortex-f4 linker script and with cortex-rt

here i have modifiednled-roulette code to verify my dma transfer

it is a mem2mem dma transfer.which transfer 20 byte of int32 array from location x to location y 

it is verifyied using led glow pattern. which in fact integrated bare metal interface with gpio registers

there will be further structured way of implementing dma transfers with self written DMA and GPIO drivers
