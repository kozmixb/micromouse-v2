#![no_std]
#![no_main]

use arduino_hal::simple_pwm::*;
use panic_halt as _;

mod motor;

#[arduino_hal::entry]
fn main() -> ! {
    let motors = motor::init();

    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    // let mut adc = arduino_hal::Adc::new(dp.ADC, Default::default());

    let timer0 = Timer1Pwm::new(dp.TC1, Prescaler::Prescale64);

    let mut led = pins.d13.into_output();

    let mut motor_left_dir = pins.d7.into_output();
    let mut motor_right_dir = pins.d8.into_output();
    let mut motor_left = pins.d9.into_output().into_pwm(&timer0);
    let mut motor_right = pins.d10.into_output().into_pwm(&timer0);

    motor_left_dir.set_high();
    motor_right_dir.set_high();
    // motor_left.enable();
    // motor_right.enable();

    let mut serial = arduino_hal::default_serial!(dp, pins, 115200);

    loop {
        for x in (0..255).chain((0..255).rev()) {
            arduino_hal::delay_ms(10);
            motor_left.set_duty(x);
            motor_right.set_duty(x);
            ufmt::uwriteln!(&mut serial, "Value: {}", x).unwrap();
        }

        led.toggle();
        arduino_hal::delay_ms(1000);
    }
}
