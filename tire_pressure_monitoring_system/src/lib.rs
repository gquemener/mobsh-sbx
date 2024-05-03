pub mod tire_pressure_monitoring_system {
    use rand::Rng;

    pub struct Alarm {
        low_pressure_threshold: f64,
        high_pressure_threshold: f64,
        sensor: Box<dyn PressureSensor>,
        alarm_on: bool,
    }

    impl Alarm {
        pub fn new() -> Self {
            Alarm {
                low_pressure_threshold: 17.0,
                high_pressure_threshold: 21.0,
                sensor: Box::new(Sensor::new()),
                alarm_on: false,
            }
        }

        pub fn check(&mut self) {
            let psi_pressure_value = self.sensor.pop_next_pressure_psi_value();

            if psi_pressure_value < self.low_pressure_threshold
                || psi_pressure_value > self.high_pressure_threshold
            {
                self.alarm_on = true;
            }
        }

        pub fn is_alarm_on(&self) -> bool {
            self.alarm_on
        }
    }

    pub struct Sensor {
        offset: f64,
    }

    impl Sensor {
        pub fn new() -> Self {
            Sensor { offset: 16.0 }
        }

        fn sample_pressure() -> f64 {
            let mut rng = rand::thread_rng();
            let pressure_telemetry_value = 6.0 * rng.gen::<f64>() * rng.gen::<f64>();
            pressure_telemetry_value
        }
    }

    impl PressureSensor for Sensor { 
        fn pop_next_pressure_psi_value(&self) -> f64 {
            let pressure_telemetry_value = Self::sample_pressure();
            self.offset + pressure_telemetry_value
        }
    }

    pub trait PressureSensor {
        fn pop_next_pressure_psi_value(&self) -> f64;
    }

    #[cfg(test)]
    mod tests {
        use crate::tire_pressure_monitoring_system::Sensor;

        use super::Alarm;

        #[test]
        fn test_alarm_by_defaut_is_off() {
            let alarm = Alarm::new();
            assert_eq!(false, alarm.is_alarm_on());
        }

        #[test]
        fn test_alarm_is_on_when_pressure_is_too_low() {
            let mut alarm = Alarm::new();
            //alarm.sensor = ??

            alarm.check();

            assert_eq!(true, alarm.is_alarm_on());
        }
    }
}
