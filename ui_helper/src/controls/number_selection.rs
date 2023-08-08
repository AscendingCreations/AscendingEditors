use iced::{theme, Element};
use iced_aw::{NumberInput, NumberInputStyles};
use num_traits::{Num, NumAssignOps};
use std::fmt::Display;
use std::marker::PhantomData;
use std::str::FromStr;

#[derive(Debug, Default)]
pub struct NumInput<V, M> {
    phantomdata: PhantomData<M>,
    pub value: V,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NumInputMessage<V> {
    Change(V),
}

impl<V> NumInputMessage<V>
where
    V: Num + NumAssignOps + PartialOrd + Display + FromStr + Copy,
{
    pub fn get_data(&self) -> V {
        let NumInputMessage::Change(data) = self;
        *data
    }
}

impl<V> NumInputMessage<V>
where
    V: Eq + Copy,
{
    pub fn get_enum(&self) -> V {
        let NumInputMessage::Change(data) = self;
        *data
    }
}

impl<V, M> NumInput<V, M>
where
    V: Num + NumAssignOps + PartialOrd + Display + FromStr + Copy,
    M: Clone,
{
    pub fn new(value: V) -> NumInput<V, M>
    where
        V: 'static,
    {
        NumInput {
            phantomdata: PhantomData,
            value,
        }
    }

    pub fn view<F>(&self, id: usize, min: V, max: V, step: V, on_change: F) -> Element<M>
    where
        F: 'static + Fn((usize, NumInputMessage<V>)) -> M + Copy,
        V: 'static,
        M: 'static,
    {
        Element::new(
            NumberInput::new(self.value, max, NumInputMessage::Change)
                .step(step)
                .min(min)
                .style(NumberInputStyles::custom(araiseal_styles::CustomNumInput)),
        )
        .map(move |i| on_change((id, i)))
    }
}
