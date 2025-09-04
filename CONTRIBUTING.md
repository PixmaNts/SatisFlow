# Contributing to SatisFlow

First off, thank you for considering contributing to SatisFlow! 🎉

Following these guidelines helps communicate that you respect the time of the developers managing and developing this open source project. In return, they should reciprocate that respect in addressing your issue, assessing changes, and helping you finalize your pull requests.

## 🤔 What kinds of contributions are we looking for?

We love to receive contributions from our community! There are many ways to contribute:

- **Bug reports and feature requests** via GitHub Issues
- **Code contributions** via Pull Requests
- **Documentation improvements**
- **Recipe data updates** for new Satisfactory content
- **Translation contributions** (future feature)
- **UI/UX improvements and suggestions**

## 🚀 Getting Started

### Development Environment Setup

1. **Prerequisites**:
   ```bash
   # Install Rust (latest stable)
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   
   # Add WASM target
   rustup target add wasm32-unknown-unknown
   
   # Install Dioxus CLI
   cargo install dioxus-cli
   ```

2. **Fork and Clone**:
   ```bash
   # Fork the repository on GitHub, then:
   git clone https://github.com/YOUR_USERNAME/SatisFlow.git
   cd SatisFlow
   ```

3. **Development Server**:
   ```bash
   # Run with hot reload for development
   dx serve --platform web --hot-reload
   ```

### Project Structure

```
src/
├── main.rs           # App entry point and main component
├── models/           # Data structures (Factory, Recipe, etc.)
├── gui/              # UI components for each view
├── engine/           # Business logic and calculations
└── data/             # External data loading utilities

public/data/          # Game data files (JSON)
design/               # Architecture and planning docs
```

## 📋 How to Contribute

### Reporting Bugs 🐛

Before creating bug reports, please check existing issues to avoid duplicates. When creating a bug report, include:

- **Clear title and description**
- **Steps to reproduce** the issue
- **Expected vs actual behavior**
- **Browser and version** (since this is a web app)
- **Screenshots** if applicable
- **Any error messages** from browser console

Use this template:
```markdown
**Bug Description**
A clear description of what the bug is.

**To Reproduce**
1. Go to '...'
2. Click on '....'
3. See error

**Expected Behavior**
What you expected to happen.

**Environment**
- Browser: [e.g., Chrome 120, Firefox 121]
- OS: [e.g., Windows 11, macOS 14]
```

### Suggesting Features ✨

We track feature requests as GitHub issues. When suggesting a feature:

- **Use a clear, descriptive title**
- **Provide detailed description** of the feature
- **Explain why this would be useful** to most users
- **Consider the scope** - is it a small enhancement or major feature?

### Contributing Code 💻

1. **Check existing issues** - look for "good first issue" or "help wanted" labels
2. **Create an issue** for new features to discuss before coding
3. **Fork the repository** and create a feature branch:
   ```bash
   git checkout -b feature/amazing-new-feature
   ```

4. **Follow our coding standards** (see below)
5. **Test your changes** thoroughly
6. **Update documentation** if needed
7. **Commit with clear messages**:
   ```bash
   git commit -m "Add factory deletion confirmation dialog
   
   - Prevents accidental factory deletion
   - Shows factory name and production line count
   - Includes 'Are you sure?' confirmation"
   ```

8. **Push and create a Pull Request**

### Pull Request Guidelines

- **Fill out the PR template** completely
- **Link to relevant issues** using "Fixes #123" or "Relates to #456"
- **Include tests** if applicable
- **Keep PRs focused** - one feature/fix per PR
- **Respond to feedback** promptly and constructively

## 🎨 Coding Standards

### Rust Code Style

- **Use `rustfmt`**: Code is automatically formatted
- **Follow Rust naming conventions**: snake_case for functions, PascalCase for types
- **Write descriptive commit messages** following conventional commits
- **Add documentation** for public functions and complex logic
- **Use meaningful variable names**: `production_line_count` not `plc`

### Component Guidelines

- **Keep components focused**: Each component should have a single responsibility
- **Use props for data**: Avoid accessing global state when possible
- **Handle loading states**: Show spinners/skeletons for async operations
- **Include error boundaries**: Graceful handling of failures

### Data Format Standards

When adding/modifying recipe data:
```json
{
  "name": "Recipe Name",
  "machine_type": "Constructor|Assembler|Manufacturer|etc",
  "base_duration": 2.0,
  "inputs": [
    {"item": "Input Item", "quantity_per_min": 30.0}
  ],
  "outputs": [
    {"item": "Output Item", "quantity_per_min": 20.0}
  ]
}
```

## 🧪 Testing

Currently manual testing is required. When contributing:

1. **Test the happy path** - normal user flows work correctly
2. **Test edge cases** - empty states, maximum values, etc.
3. **Test across browsers** - Chrome, Firefox, Safari if possible
4. **Test responsiveness** - mobile and desktop viewports
5. **Check performance** - no significant slowdowns with large datasets

Future: We plan to add automated testing with component tests and E2E testing.

## 📝 Documentation

- **Update README.md** for user-facing changes
- **Update design docs** for architectural changes
- **Comment complex algorithms** in code
- **Update CHANGELOG.md** following [Keep a Changelog](https://keepachangelog.com/) format

## 💬 Community Guidelines

- **Be respectful and inclusive** - we welcome contributors of all backgrounds
- **Assume good intentions** - we're all here to improve the project
- **Give constructive feedback** - specific, actionable suggestions
- **Be patient** - maintainers contribute in their free time
- **Ask for help** - don't struggle in silence, we're here to help!

## 📞 Getting Help

- **GitHub Discussions** - for general questions and ideas
- **GitHub Issues** - for bug reports and feature requests
- **Email**: maxime@maxime.pointet.fr - for private/sensitive matters

## 🏷️ Labels

We use these labels to categorize issues and PRs:

- `bug` - Something isn't working correctly
- `enhancement` - New feature or improvement
- `good first issue` - Perfect for newcomers
- `help wanted` - We need community help on this
- `documentation` - Related to docs
- `performance` - Performance improvements
- `ui/ux` - User interface and experience
- `recipe-data` - Satisfactory game data updates

## 🎯 Priorities

Current high-priority areas where contributions are especially welcome:

1. **Factory creation dialogs** - Make it easier to set up new factories
2. **Recipe database expansion** - Add missing Satisfactory recipes
3. **UI polish** - Improve the user experience and visual design
4. **Performance optimization** - Handle larger datasets smoothly
5. **Mobile responsiveness** - Better mobile user experience

---

Thank you for contributing to SatisFlow! 🚀

*This document is inspired by the [Atom](https://github.com/atom/atom/blob/master/CONTRIBUTING.md) and [Rails](https://github.com/rails/rails/blob/main/CONTRIBUTING.md) contributing guides.*
