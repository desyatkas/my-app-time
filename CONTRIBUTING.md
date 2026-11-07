# Contributing

Thanks for taking the time to contribute! Open source thrives on community input, so whether you're fixing a typo, squashing a bug, or adding a new feature - you're awesome. This guide will help you get started.

## How to Contribute

It's pretty simple:
1. Fork the repository
2. Create your branch (`<type>/<description>`)
    ```sh
    git checkout -b feat/my-cool-feature
    ```
3. Make changes
4. Commit
    ```sh
    git commit -m "feat: add cool button"
    ```
5. Push
    ```sh
    git push origin feat/my-cool-feature
    ```
6. Open a pull request with a clear description

We'll review, discuss if needed, and merge once it's ready.

## Commit Messages

We follow the [Conventional Commits](https://www.conventionalcommits.org) standard, so your commit messages should follow this format: `<type>[optional scope]: <description>`.

Preferred type labels and their use cases:
* `build`: affects the build system or external dependencies
* `chore`: general maintenance
* `ci`: changes CI configuration or scripts
* `docs`: improves and/or fixes documentation
* `feat`: introduces/updates a feature
* `fix`: patches a bug
* `perf`: improves performance
* `refactor`: restructures or improves code without changing behavior
* `revert`: reverts a previous commit
* `style`: formatting without affecting code behavior
* `test`: adds or improves tests

These labels also apply to your branch names:
* `feat/add-cool-button`
* `fix/button-not-working`
* `docs/update-readme`

## Pull Request Guidelines

* Keep your PR focused on a single change
* Keep your PR description short but informative
* Update documentation when necessary
* Make sure it builds and runs as expected

## License

By contributing, you agree that your contributions will be licensed under the terms of the project's [GPL-3.0-or-later](LICENSE) license.
