use embedded_hal::digital::{ErrorType, OutputPin, StatefulOutputPin};

use crate::{
    ActiveHigh, ActiveLow, OutputSwitch, StatefulOutputSwitch, Switch, ToggleableOutputSwitch,
};

impl<T: OutputPin> OutputSwitch for Switch<T, ActiveHigh> {
    type Error = <T as ErrorType>::Error;

    fn on(&mut self) -> Result<(), Self::Error> {
        self.pin.set_high()
    }

    fn off(&mut self) -> Result<(), Self::Error> {
        self.pin.set_low()
    }
}

impl<T: OutputPin> OutputSwitch for Switch<T, ActiveLow> {
    type Error = <T as ErrorType>::Error;

    fn on(&mut self) -> Result<(), Self::Error> {
        self.pin.set_low()
    }

    fn off(&mut self) -> Result<(), Self::Error> {
        self.pin.set_high()
    }
}

impl<T: OutputPin + StatefulOutputPin, ActiveLevel> ToggleableOutputSwitch
    for Switch<T, ActiveLevel>
{
    type Error = <T as ErrorType>::Error;

    fn toggle(&mut self) -> Result<(), Self::Error> {
        self.pin.toggle()
    }
}

impl<T: OutputPin + StatefulOutputPin> StatefulOutputSwitch for Switch<T, ActiveLow> {
    type Error = <T as ErrorType>::Error;

    fn is_on(&mut self) -> Result<bool, Self::Error> {
        self.pin.is_set_low()
    }

    fn is_off(&mut self) -> Result<bool, Self::Error> {
        self.pin.is_set_high()
    }
}

impl<T: OutputPin + StatefulOutputPin> StatefulOutputSwitch for Switch<T, ActiveHigh> {
    type Error = <T as ErrorType>::Error;

    fn is_on(&mut self) -> Result<bool, Self::Error> {
        self.pin.is_set_high()
    }

    fn is_off(&mut self) -> Result<bool, Self::Error> {
        self.pin.is_set_low()
    }
}
