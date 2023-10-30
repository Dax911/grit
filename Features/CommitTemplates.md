# Templating System for Commits Specification for GRIT

## Overview

Provide a system that allows users to define and use templates for commit messages.

## Features

- **Template Creation**: Users can create and save their own commit message templates.
- **Template Usage**: Users can apply templates when making commits.
- **Variable Substitution**: Support for variables in templates that can be substituted at commit time.

## Implementation Details

- Store templates in a designated directory within the repository.
- Use a simple templating language to define templates and variables.
- Implement a command-line interface to interact with templates.

## Future Enhancements

- Add support for global templates that can be used across multiple repositories.
- Integrate with popular editors for template selection and usage.
