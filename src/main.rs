extern crate rand;

use rand::Rng;

#[derive(Debug)]
struct LinearRegression {
    weight: f32,
    bias: f32,
}

impl LinearRegression {
    // Constructor to initialize the model with small weights and biases
    fn new() -> LinearRegression {
        let mut rng = rand::rng();
        LinearRegression {
            weight: rng.random_range(-0.1..0.1),
            bias: rng.random_range(-0.1..0.1),
        }
    }

    // Forward pass: Calculate the predicted values based on input x
    fn predict(&self, x: f32) -> f32 {
        self.weight * x + self.bias
    }

    // Training function
    fn train_model(&mut self, xs: Vec<f32>, targets: Vec<f32>, epochs: usize, lr: f32) {
        let n = xs.len() as f32;
        for epoch in 0..epochs {
            let mut total_loss = 0.0;

            // Calculate the predictions and compute loss
            let predictions: Vec<f32> = xs.iter().map(|&x| self.predict(x)).collect();
            let errors: Vec<f32> = predictions.iter().zip(&targets).map(|(&pred, &target)| pred - target).collect();

            // Loss calculation (Mean Squared Error)
            for (pred, target) in predictions.iter().zip(&targets) {
                total_loss += (pred - target).powi(2);
            }
            total_loss /= n;

            // Gradient calculation for weight and bias
            let grad_weight = 2.0 / n * xs.iter().zip(&errors).map(|(&x, &err)| x * err).sum::<f32>();
            let grad_bias = 2.0 / n * errors.iter().sum::<f32>();

            // Clip gradients to avoid explosion
            let grad_weight = grad_weight.max(-5.0).min(5.0);  // Clip between -5 and 5
            let grad_bias = grad_bias.max(-5.0).min(5.0);      // Clip between -5 and 5

            // Update parameters
            self.weight -= lr * grad_weight;
            self.bias -= lr * grad_bias;

            // Print the loss for every 10th epoch for debugging
            if epoch % 10 == 0 {
                println!("Epoch {}: Loss = {}", epoch, total_loss);
            }

            // Check for divergence
            if total_loss.is_nan() || total_loss.is_infinite() {
                println!("Warning: Loss became NaN or Infinite. Stopping training.");
                break;
            }
        }
    }
}

fn main() {
    // Example data (x, y) points
    let x_train = vec![0.0, 1.0, 2.0, 3.0, 4.0];  // Input data (x values)
    let y_train = vec![1.8645625, 2.3195887, 5.9059525, 7.8615713, 9.735073];  // Target values (y values)

    // Create a new model instance
    let mut model = LinearRegression::new();

    println!("Training the model...");
    // Train the model with the given data
    model.train_model(x_train.clone(), y_train.clone(), 50, 0.0001);  // Use a smaller learning rate for stability

    // Test predictions after training
    println!("\nLinear Regression Predictions:");
    for x in 0..5 {
        let prediction = model.predict(x as f32);
        println!("x: {}, predicted y: {}", x, prediction);
    }
}
