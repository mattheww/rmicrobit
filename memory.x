MEMORY
{
  /* See https://infocenter.nordicsemi.com/pdf/nRF51_RM_v3.0.pdf 5.1 for base
     addresses.

     nrf51822 may have 128k or 256k flash; the micro:bit has 256k.
     nrf51822 may have 16k or 32k RAM; the micro:bit has 16k.
  */
  FLASH : ORIGIN = 0x00000000, LENGTH = 256K
  RAM : ORIGIN = 0x20000000, LENGTH = 16K
}
