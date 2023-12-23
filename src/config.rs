const BAUDRATE: u32 = 115200;

// Motors
const MOTOR_LEFT_DIR: u8 = 7;
const MOTOR_RIGHT_DIR: u8 = 8;
const MOTOR_LEFT_PWM: u8 = 9;
const MOTOR_RIGHT_PWM: u8 = 10;
const MOTOR_LEFT_POLARITY: i8 = (-1);
const MOTOR_RIGHT_POLARITY: i8 = (-1);

// Indicators
const LED_RIGHT: u8 = 6;
const LED_LEFT: u8 = 11;

// Emitters
const EMITTER = 12;

// Sensors
const LINE_SENSOR_RIGHT = A0;
const LINE_SENSOR_FRONT_RIGHT = A1;
const LINE_SENSOR_FRONT_LEFT = A2;
const LINE_SENSOR_LEFT = A3;
const SENSOR_4 = A4;
const SENSOR_5 = A5;

const FUNCTION_PIN = A6;
const BATTERY_PIN = A7;

// Battery
const MAX_MOTOR_VOLTS: f32 = 6.0;
const BATTERY_R1: f32 = 10000.0;  // resistor to battery +
const BATTERY_R2: f32 = 10000.0;  // resistor to Gnd
const BATTERY_DIVIDER_RATIO: f32 =
    BATTERY_R2 / (BATTERY_R1 + BATTERY_R2);
const ADC_FSR: f32 = 1023.0;       // The maximum reading for the ADC
const ADC_REF_VOLTS: f32 = 4.987;  // Reference voltage of ADC

const BATTERY_MULTIPLIER: f32 =
    (ADC_REF_VOLTS / ADC_FSR / BATTERY_DIVIDER_RATIO);

const WHEEL_DIAMETER: f32 = 31.966;  // 33.298
const WHEEL_CIRCUMFENCE: f32 = WHEEL_DIAMETER * PI;

const MOUSE_RADIUS: f32 = 37.92;  // 39.50;
const MOUSE_CIRCUMFENCE: f32 = 2 * PI * MOUSE_RADIUS;
