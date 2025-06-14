# p-multisig

This is a program built using Pinocchio to create a multisig for executing arbitrary transactions on Solana.

## Project Structure

- **`src/entrypoint.rs`**: Contains the program entrypoint.
- **`src/instruction`**: Contains the program instructions and instruction handlers.
- **`src/state`**: Contains the program state.
- **`src/constants.rs`**: Contains constant values.
- **`src/error.rs`**: Contains custom errors.
- **`tests/tests.rs`**: Includes integration tests for the program using the `mollusk_svm` framework.

## Prerequisites

- Rust and Cargo
- Solana CLI tools
- Pinocchio
- bytemuck (for data serialization/deserialization)
- Mollusk (for testing)

## Build the Program

To build the program, run the following command:

```bash
make build
```

## Test the Program

To test the program, run the following command:

```bash
make test
```

## Get the Program key

To get the program key, run the following command:

```bash
make test
```

Replace the key inside the `declare_id!("..")` macro with your key.


## Deploy the Program

To deploy the program, run the following command:

```bash
make deploy
```