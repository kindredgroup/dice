#[derive(Clone, Debug)]
pub struct UnivariateDescentConfig {
    pub init_value: f64,
    pub init_step: f64,
    pub min_step: f64,
    pub max_steps: u64,
    pub acceptable_residual: f64,
}

impl UnivariateDescentConfig {
    fn validate(&self) {
        assert!(self.min_step > 0.0, "min step must be positive");
        assert!(self.acceptable_residual >= 0.0, "acceptable residual must be non-negative");
    }
}

#[derive(Debug)]
pub struct UnivariateDescentOutcome {
    pub steps: u64,
    pub optimal_value: f64,
    pub optimal_residual: f64,
}

/// Univariate, derivative-free search.
pub fn univariate_descent(
    config: &UnivariateDescentConfig,
    mut loss_f: impl FnMut(f64) -> f64,
) -> UnivariateDescentOutcome {
    config.validate();

    let mut steps = 0;
    let mut residual = loss_f(config.init_value);
    if residual <= config.acceptable_residual {
        return UnivariateDescentOutcome {
            steps: 0,
            optimal_value: config.init_value,
            optimal_residual: residual
        };
    }

    let (mut value, mut step) = (config.init_value, config.init_step);
    let (mut optimal_value, mut optimal_residual) = (value, residual);
    while steps < config.max_steps {
        steps += 1;
        let new_value = value + step;
        let new_residual = loss_f(new_value);

        if new_residual > residual {
            step = -step * 0.5;
            if step.abs() < config.min_step {
                break;
            }
        } else if new_residual < optimal_residual {
            optimal_residual = new_residual;
            optimal_value = new_value;

            if optimal_residual <= config.acceptable_residual {
                break;
            }
        }
        residual = new_residual;
        value = new_value;
    }
    UnivariateDescentOutcome {
        steps,
        optimal_value,
        optimal_residual,
    }
}

#[cfg(test)]
mod tests {
    use assert_float_eq::assert_float_absolute_eq;
    use crate::opt::{univariate_descent, UnivariateDescentConfig};

    #[test]
    fn univariate_descent_sqrt() {
        let config = UnivariateDescentConfig {
            init_value: 0.0,
            init_step: 0.1,
            min_step: 0.00001,
            max_steps: 100,
            acceptable_residual: 0.0
        };
        let outcome = univariate_descent(&config, |value| (81.0 - value.powi(2)).powi(2));
        assert_float_absolute_eq!(9.0, outcome.optimal_value, config.min_step);
    }
}