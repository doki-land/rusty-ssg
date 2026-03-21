# Unify Compiler READMEs - Product Requirement Document

## Overview
- **Summary**: Rewrite README files for multiple SSG compilers (astro, eleventy, gatsby, hexo, hugo, jekyll, mkdocs, vitepress, vuepress) to follow a unified structure and community standards in English.
- **Purpose**: Create consistent, informative, and engaging READMEs that provide clear documentation for each compiler while maintaining a cohesive look and feel across all projects.
- **Target Users**: Developers and contributors to the Rusty SSG project, as well as end users looking to use these compilers.

## Goals
- Create a unified README structure that all compilers follow
- Use English for all README content
- Include Mermaid architecture diagrams for each compiler
- Provide example project structures, config examples, and post examples
- Ensure installation instructions use `cargo install compiler-name` format
- Avoid absolute paths to prevent broken links after repository push
- Note that 100% compatibility only exists without dynamic features
- Use lively, engaging language while maintaining professional documentation standards

## Non-Goals (Out of Scope)
- Modifying the actual compiler code or functionality
- Creating new features for the compilers
- Changing the project structure of the compilers themselves
- Translating READMEs into languages other than English

## Background & Context
- The current README files for each compiler are inconsistent in structure, language, and level of detail
- Some READMEs may contain absolute paths that would break when pushed to a repository
- None of the current READMEs include Mermaid architecture diagrams
- Installation instructions may not follow the standard `cargo install` format
- The compatibility note about dynamic features is not consistently communicated

## Functional Requirements
- **FR-1**: Each README must follow a unified structure including sections for overview, installation, usage, project structure, configuration, examples, and contribution guidelines
- **FR-2**: Each README must include a Mermaid architecture diagram showing the compiler's core components and workflow
- **FR-3**: Each README must provide an example project structure, configuration example, and post example
- **FR-4**: Installation instructions must use the `cargo install compiler-name` format
- **FR-5**: All paths in READMEs must be relative to avoid broken links
- **FR-6**: Each README must include a note that 100% compatibility only exists without dynamic features

## Non-Functional Requirements
- **NFR-1**: READMEs must use lively, engaging language while maintaining professional documentation standards
- **NFR-2**: READMEs must follow community standards for Rust projects
- **NFR-3**: READMEs must be clear, concise, and easy to navigate
- **NFR-4**: Mermaid diagrams must be well-structured and accurately represent the compiler's architecture

## Constraints
- **Technical**: Must use Mermaid syntax for diagrams, must not use absolute paths
- **Business**: Must maintain consistency across all compiler READMEs
- **Dependencies**: Relies on Mermaid being supported by the platform where READMEs are viewed

## Assumptions
- Mermaid diagrams will be rendered correctly in the platform where READMEs are viewed (e.g., GitHub)
- Users have basic knowledge of Rust and cargo
- The target audience is familiar with static site generators

## Acceptance Criteria

### AC-1: Unified Structure
- **Given**: A compiler README file
- **When**: A user opens the README
- **Then**: The README follows the unified structure with all required sections
- **Verification**: `human-judgment`
- **Notes**: Structure includes Overview, Installation, Usage, Project Structure, Configuration, Examples, Contribution Guidelines

### AC-2: Mermaid Architecture Diagram
- **Given**: A compiler README file
- **When**: A user views the README
- **Then**: The README includes a Mermaid diagram showing the compiler's architecture
- **Verification**: `human-judgment`
- **Notes**: Diagram should show core components and workflow

### AC-3: Example Project Structure
- **Given**: A compiler README file
- **When**: A user reads the README
- **Then**: The README includes an example project structure
- **Verification**: `human-judgment`

### AC-4: Configuration Example
- **Given**: A compiler README file
- **When**: A user reads the README
- **Then**: The README includes a configuration example
- **Verification**: `human-judgment`

### AC-5: Post Example
- **Given**: A compiler README file
- **When**: A user reads the README
- **Then**: The README includes a post example
- **Verification**: `human-judgment`

### AC-6: Installation Instructions
- **Given**: A compiler README file
- **When**: A user reads the installation section
- **Then**: The README includes instructions using `cargo install compiler-name`
- **Verification**: `human-judgment`

### AC-7: Relative Paths Only
- **Given**: A compiler README file
- **When**: A user reviews the README
- **Then**: The README contains only relative paths, no absolute paths
- **Verification**: `programmatic`
- **Notes**: Can be verified by searching for absolute path patterns

### AC-8: Compatibility Note
- **Given**: A compiler README file
- **When**: A user reads the README
- **Then**: The README includes a note that 100% compatibility only exists without dynamic features
- **Verification**: `human-judgment`

### AC-9: Lively Language
- **Given**: A compiler README file
- **When**: A user reads the README
- **Then**: The README uses lively, engaging language while maintaining professionalism
- **Verification**: `human-judgment`

## Open Questions
- [ ] Should each compiler's README include specific details unique to that compiler, or should all READMEs be identical except for compiler-specific information?
- [ ] What level of technical detail should be included in the architecture diagrams?
- [ ] Should the examples be identical across all compilers or tailored to each compiler's specific features?