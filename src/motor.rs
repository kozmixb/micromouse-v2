use arduino_hal::{
    port::{
        mode::{self},
        Pin,
    },
    simple_pwm::*,
};

pub struct Motor {
    speed: Pin<mode::PwmOutput<Timer1Pwm>>,
    direction: Pin<mode::Output>,
}

pub struct Motors {
    left: Motor,
    right: Motor,
}

impl Motors {
    pub fn new(l: Motor, r: Motor) -> Self {
        Motors { left: l, right: r }
    }
}

pub fn init() -> Motors {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let timer0 = Timer1Pwm::new(dp.TC1, Prescaler::Prescale64);

    let mut motor_left_dir = pins.d7.into_output();
    let mut motor_right_dir = pins.d8.into_output();
    let mut motor_left = pins.d9.into_output().into_pwm(&timer0);
    let mut motor_right = pins.d10.into_output().into_pwm(&timer0);

    Motors::new(
        Motor {
            speed: motor_left,
            direction: motor_left_dir,
        },
        Motor {
            speed: motor_right,
            direction: motor_right_dir,
        },
    )
}
