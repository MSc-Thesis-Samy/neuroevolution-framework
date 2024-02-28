use std::f64::consts::PI;
use crate::network_trait::NetworkTrait;

const UNIT_CIRCLE_STEPS: u32 = 1000;

pub fn half<N, const M: usize>(network: &N) -> f64
where
    N: NetworkTrait<M,2>,
{
    let mut sum = 0;
    for i in 0..UNIT_CIRCLE_STEPS {
        let angle = 2. * PI * i as f64 / UNIT_CIRCLE_STEPS as f64;
        let output = network.evaluate(&[1., angle]);
        if output && angle < PI || !output && angle > PI {
            sum += 1;
        }
    }
    sum as f64 / UNIT_CIRCLE_STEPS as f64
}

pub fn quarter<N, const M: usize>(network: &N) -> f64
where
    N: NetworkTrait<M,2>,
{
    let mut sum = 0;
    for i in 0..UNIT_CIRCLE_STEPS {
        let angle = 2. * PI * i as f64 / UNIT_CIRCLE_STEPS as f64;
        let output = network.evaluate(&[1., angle]);
        if output && angle < PI / 2. || !output && angle > PI / 2. {
            sum += 1;
        }
    }
    sum as f64 / UNIT_CIRCLE_STEPS as f64
}

pub fn two_quarters<N, const M: usize>(network: &N) -> f64
where
    N: NetworkTrait<M,2>,
{
    let mut sum = 0;
    for i in 0..UNIT_CIRCLE_STEPS {
        let angle = 2. * PI * i as f64 / UNIT_CIRCLE_STEPS as f64;
        let output = network.evaluate(&[1., angle]);
        if output && (angle < PI / 2. || angle > PI && angle < 3. * PI / 2.)
        || !output && (angle > PI / 2. && angle < PI || angle > 3. * PI / 2.) {
            sum += 1;
        }
    }
    sum as f64 / UNIT_CIRCLE_STEPS as f64
}

pub fn square<N, const M: usize>(network: &N) -> f64
where
    N: NetworkTrait<M,2>,
{
    let points_with_labels = [
        (1., PI / 4., true),
        (1., 3. * PI / 4., false),
        (1., 5. * PI / 4., true),
        (1., 7. * PI / 4., false),
    ];

    points_with_labels
        .iter()
        .map(|&(r, theta, label)| {
            let output = network.evaluate(&[r, theta]);
            if output && label || !output && !label {
                1.
            } else {
                0.
            }
        })
        .sum::<f64>() / 4.
}

pub fn cube<N, const M: usize>(network: &N) -> f64
where
    N: NetworkTrait<M,3>,
{
    let points_with_labels = [
        (1., PI / 4., PI / 4., true),
        (1., 3. * PI / 4., PI / 4., false),
        (1., 5. * PI / 4., PI / 4., true),
        (1., 7. * PI / 4., PI / 4., false),
        (1., PI / 4., 3. * PI / 4., true),
        (1., 3. * PI / 4., 3. * PI / 4., false),
        (1., 5. * PI / 4., 3. * PI / 4., true),
        (1., 7. * PI / 4., 3. * PI / 4., false),
    ];

    points_with_labels
        .iter()
        .map(|&(r, theta, phi, label)| {
            let output = network.evaluate(&[r, theta, phi]);
            if output && label || !output && !label {
                1.
            } else {
                0.
            }
        })
        .sum::<f64>() / 8.
}
