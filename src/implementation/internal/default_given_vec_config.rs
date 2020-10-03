use crate::{DefaultExcessShrink, GivenVecConfig, GivenVecOptimization};

pub struct DefaultGivenVecConfig;

impl GivenVecConfig for DefaultGivenVecConfig {
    type TExcessShrink = DefaultExcessShrink;

    fn optimization() -> GivenVecOptimization {
        GivenVecOptimization::Operations
    }
}
