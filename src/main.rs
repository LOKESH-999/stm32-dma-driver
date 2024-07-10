//#![deny(unsafe_code)]
#![no_main]
#![no_std]
mod dma;
use aux5::{entry, Delay, DelayMs, LedArray, OutputSwitch};

#[entry]
fn main() -> ! {
    let (mut delay, mut leds): (Delay, LedArray) = aux5::init();

    let half_period = 50_u8;
    leds[0].on().ok();
    leds[1].on().ok();
    leds[2].on().ok();
    leds[3].on().ok();
    leds[4].on().ok();
    leds[5].on().ok();
    leds[6].on().ok();
    leds[7].on().ok();
    delay.delay_ms(2500u16);
    leds[0].off().ok();
    leds[1].off().ok();
    leds[2].off().ok();
    leds[3].off().ok();
    leds[4].off().ok();
    leds[5].off().ok();
    leds[6].off().ok();
    leds[7].off().ok();
    let ptr=0x4800_1014 as *mut u32;

    let mut val;
   unsafe{ 
	let adr=0x4800_1000 as *mut u32;
	core::ptr::write_volatile(adr,0x55555555);	
    }
   //power enable for tim6
   let port=0x4002_1000;
   unsafe{
	let en=core::ptr::read_volatile((port+0x1c) as *mut u32);
   	core::ptr::write_volatile((port+0x1c) as *mut u32,en|1<<4);
   }
   unsafe{
   let base_addr=0x4000_1000;
   let psc=base_addr + 0x28;
   let dma=0x0c;	
   //setting cr1
   core::ptr::write_volatile(base_addr as *mut u16,0x8d);
   //MMS setting reset
   core::ptr::write_volatile((base_addr  as *mut u16).offset(0x04),0x10);
   //DMA setting ->intrupt enable and dma enabl
   core::ptr::write_volatile((base_addr + 0x0c) as *mut u16,0x101);
   //PSC setting=7999+1 for milli_second
   core::ptr::write_volatile((base_addr + 0x28) as *mut u16,7200);
   }		
   dma::simp();
//   let port=0x4002_1000;
   
    loop {
	for i in 0..8{
		
		unsafe{
			if i==0{val=9}
			else if i==1{val=8}
			else if i==2{val=10}
			else if i==3{val=15}
			else if i==4{val=11}
			else if i==5{val=14}
			else if i==6{val=12}
			else {val=13}
	         	core::ptr::write_volatile(ptr,1<<val);
			delay_ms(1000);
//        		delay.delay_ms(half_period);
//			core::ptr::write_volatile(ptr,0<<val);
//	        	delay.delay_ms(half_period);
		}	
	}
//	unsafe{core::ptr::write_volatile((port | 0x14) as *mut u32,0);}  //disable whole AHBENR
    }
}




pub fn delay_ms(t:u16){
   let base_addr=0x4000_1000;
   let arr=0x2c;
   let sr=0x10;

   unsafe{
   // counter enable 
   core::ptr::write_volatile(base_addr as *mut u16,0x8d);
   //arr register
   core::ptr::write_volatile((base_addr + arr) as *mut u16,t-1);

   //EGR -> re init
//   core::ptr::write_volatile((base_addr + 0x14) as * mut u16,1);
 
   // enable counter
//   core::ptr::write_volatile(base_addr as *mut u16,);
//   if core::ptr::read_volatile((base_addr+
   
   //looping untill clock gets trigered
   while core::ptr::read_volatile((base_addr +sr) as *mut u16)==0{}
   
   //clearing the status register
   core::ptr::write_volatile((base_addr+sr) as * mut u16,0);   
   }
}
