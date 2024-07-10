pub const DMA1_BASE:u32=0x4002_0000;
pub const DMA1_ISR:u32=DMA1_BASE+0x00;
pub const DMA1_IFRC:u32=DMA1_BASE+0x04;
//config register
pub const DMA1_CCR1:u32 = DMA1_BASE+0x08+(0x14*0);
pub const DMA1_CCR2:u32 = DMA1_BASE+0x08+(0x14*1);
pub const DMA1_CCR3:u32 = DMA1_BASE+0x08+(0x14*2);
pub const DMA1_CCR4:u32 = DMA1_BASE+0x08+(0x14*3);
pub const DMA1_CCR5:u32 = DMA1_BASE+0x08+(0x14*4);
pub const DMA1_CCR6:u32 = DMA1_BASE+0x08+(0x14*5);
pub const DMA1_CCR7:u32 = DMA1_BASE+0x08+(0x14*6);
//countdownregister
pub const DMA1_CNDTR1:u32 = DMA1_BASE+0x0C+(0x14*0);
pub const DMA1_CNDTR2:u32 = DMA1_BASE+0x0C+(0x14*1);
pub const DMA1_CNDTR3:u32 = DMA1_BASE+0x0C+(0x14*2);
pub const DMA1_CNDTR4:u32 = DMA1_BASE+0x0C+(0x14*3);
pub const DMA1_CNDTR5:u32 = DMA1_BASE+0x0C+(0x14*4);
pub const DMA1_CNDTR6:u32 = DMA1_BASE+0x0C+(0x14*5);
pub const DMA1_CNDTR7:u32 = DMA1_BASE+0x0C+(0x14*6);
//peripheral addr
pub const DMA_CPAR1:u32 = DMA1_BASE+0x10+(0x14*0);
pub const DMA_CPAR2:u32 = DMA1_BASE+0x10+(0x14*1);
pub const DMA_CPAR3:u32 = DMA1_BASE+0x10+(0x14*2);
pub const DMA_CPAR4:u32 = DMA1_BASE+0x10+(0x14*3);
pub const DMA_CPAR5:u32 = DMA1_BASE+0x10+(0x14*4);
pub const DMA_CPAR6:u32 = DMA1_BASE+0x10+(0x14*5);
pub const DMA_CPAR7:u32 = DMA1_BASE+0x10+(0x14*6);
//memory addr
pub const DMA_CMAR1:u32 = DMA1_BASE+0x14+(0x14*0);
pub const DMA_CMAR2:u32 = DMA1_BASE+0x14+(0x14*1);
pub const DMA_CMAR3:u32 = DMA1_BASE+0x14+(0x14*2);
pub const DMA_CMAR4:u32 = DMA1_BASE+0x14+(0x14*3);
pub const DMA_CMAR5:u32 = DMA1_BASE+0x14+(0x14*4);
pub const DMA_CMAR6:u32 = DMA1_BASE+0x14+(0x14*5);
pub const DMA_CMAR7:u32 = DMA1_BASE+0x14+(0x14*6);

//#[repr(transparent)]
pub enum CHANNEL{
	CH1,
	CH2,
	CH3,
	CH4,
	CH5,
	CH6,
	CH7
}

#[repr(transparent)]
pub enum DMA_ERR{
	CHANNEL_IN_USE,
}

#[repr(transparent)]
pub struct DMA1{
	channels:CHANNEL
}

impl DMA1{
	pub fn new(ch:CHANNEL,p_addr:*mut u32,m_addr:*mut u32)->Result<DMA1,DMA_ERR>{
		todo!();
	}

}
pub fn simp(){
	//rcc base
	let rcc=0x4002_1000;
	unsafe{
		let r=core::ptr::read_volatile((rcc+0x14) as *const u32);
		core::ptr::write_volatile((rcc+0x14) as *mut u32,r|1);
	}
	
	let mut s=[1,2,3,4,5];
	let mut d=[0,0,0,0,0];
	let s_addr=(&mut s as *mut i32) as u32;
	let d_addr=(&mut d as *mut i32) as u32;
	
	unsafe{
		core::ptr::write_volatile(DMA1_CCR1 as *mut u32,0);
		core::ptr::write_volatile(DMA1_IFRC as *mut u32,0);
		/*
		mem2mem			->1[14]
		priority_lvl	->(high)1[13:12]
		memsize			->(32bit)10[11:01]
		psize			->(32bit)10[9:8]
		minc			->1[7]
		pinc			->1[6]
		circ			->0[5]
		dir				->1[4]
		teie,htie,tcie	->0[3,2,1]
		chen			->1
		*/
		//count down value
		core::ptr::write_volatile(DMA1_CNDTR1 as *mut u32,5u32);
		//destination address
		core::ptr::write_volatile(DMA_CPAR1 as *mut u32,d_addr);
		//source addr
		core::ptr::write_volatile(DMA_CMAR1 as *mut u32,s_addr);
		//config values 0x00006AD1   -> and enable
		core::ptr::write_volatile(DMA1_CCR1 as *mut u32,0x00006AD1);
		delay_ms(1500);
		test(&mut s as *mut i32,&mut d as *mut i32);
	}
	
}

fn test(s:*mut i32,d:*mut i32){
	unsafe{
		let s_a=*(s as *const [i32;5]);
		let d_a=*(d as *const [i32;5]);
		if s_a==d_a{
			succes_sequence();
		}else{
			failure_sequence();
		}
	}
}
fn end_sequence(){		//by setting odr {built in leds}
	let ptr=0x4800_1014 as *mut u32;
	unsafe{
		delay_ms(1234);
		core::ptr::write_volatile(ptr,0x0000FF00);
		delay_ms(1500);
		core::ptr::write_volatile(ptr,0);
		
	}
}
fn succes_sequence(){
	let ptr=0x4800_1014 as *mut u32;
	let mut val;
	for _ in 1..5{
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
				delay_ms(250);
			}	
		}
	}
	//sequence end -->odr_high
	end_sequence();
}
fn failure_sequence(){
	let ptr=0x4800_1014 as *mut u32;
	let mut val;
	for _ in 1..5{
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
			}	
		}
	}
	//sequence end -->odr_high
	end_sequence();
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
