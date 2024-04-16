#[cfg(feature = "async")]
use crate::WaitableInputSwitch;
use crate::{ActiveHigh, ActiveLow, InputSwitch, Switch};
use embedded_hal::digital::{ErrorType, InputPin};
#[cfg(feature = "async")]
use embedded_hal_async::digital::Wait;

impl<T: InputPin> InputSwitch for Switch<T, ActiveHigh> {
    type Error = <T as ErrorType>::Error;

    fn is_active(&mut self) -> Result<bool, Self::Error> {
        self.pin.is_high()
    }
}

impl<T: InputPin> InputSwitch for Switch<T, ActiveLow> {
    type Error = <T as ErrorType>::Error;

    fn is_active(&mut self) -> Result<bool, Self::Error> {
        self.pin.is_low()
    }
}

#[cfg(feature = "async")]
impl<T: Wait + InputPin> WaitableInputSwitch for Switch<T, ActiveHigh>
where
    Switch<T, ActiveHigh>: InputSwitch,
{
    type Error = <T as ErrorType>::Error;

    async fn wait_for_active(&mut self) -> Result<(), Self::Error> {
        self.pin.wait_for_high().await
    }

    async fn wait_for_inactive(&mut self) -> Result<(), Self::Error> {
        self.pin.wait_for_low().await
    }

    async fn wait_for_change(&mut self) -> Result<(), Self::Error> {
        if self.pin.is_high()? {
            self.pin.wait_for_low().await
        } else {
            self.pin.wait_for_high().await
        }
    }
}

#[cfg(feature = "async")]
impl<T: Wait + InputPin> WaitableInputSwitch for Switch<T, ActiveLow>
where
    Switch<T, ActiveHigh>: InputSwitch,
{
    type Error = <T as ErrorType>::Error;

    async fn wait_for_active(&mut self) -> Result<(), Self::Error> {
        self.pin.wait_for_low().await
    }

    async fn wait_for_inactive(&mut self) -> Result<(), Self::Error> {
        self.pin.wait_for_high().await
    }

    async fn wait_for_change(&mut self) -> Result<(), Self::Error> {
        if self.pin.is_high()? {
            self.pin.wait_for_low().await
        } else {
            self.pin.wait_for_high().await
        }
    }
}
