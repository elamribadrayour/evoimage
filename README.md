# Genetic Algorithm Image Evolution

This project implements a genetic algorithm to evolve images towards a target image. The algorithm uses concepts such as mutation, crossover, and selection to iteratively improve a population of images.

## Features

- Configurable parameters for mutation rate, selection rate, crossover rate, and more.
- Parallel processing using Rayon for efficient computation.
- Logging for tracking the progress of the algorithm.
- Saves the best image of each epoch to a cache directory.

## Usage

1. **Configuration**: Modify the `Config` struct in `src/main.rs` to set your desired parameters.
2. **Run the Algorithm**: Execute the program using `cargo run`.
3. **View Results**: The best image of each epoch is saved in the `.cache/images/` directory.

## Example Result

Below is an example of the algorithm's output over time:

<p align="center">
    <img src="assets/result.gif" width="500" height="300" />
</p>

## Dependencies

- [Rayon](https://crates.io/crates/rayon) for parallel processing.
- [Image](https://crates.io/crates/image) for image manipulation.
- [Rand](https://crates.io/crates/rand) for random number generation.
- [Simple Logger](https://crates.io/crates/simple_logger) for logging.

## License

This project is licensed under the [WTFPL](LICENSE).
