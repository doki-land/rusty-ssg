# Rewrite Compiler READMEs - Product Requirement Document

## Overview
- **Summary**: Rewrite README files for multiple SSG compilers (astro, eleventy, gatsby, hexo, hugo, jekyll, mkdocs, vitepress, vuepress) to follow a unified structure while adapting to each compiler's specific features.
- **Purpose**: Create consistent, informative, and accurate READMEs that reflect each compiler's unique characteristics and configuration formats.
- **Target Users**: Developers using the Rust reimplementations of these static site generators.

## Goals
- Create consistent README structure across all compilers
- Adapt configuration examples to each compiler's specific format
- Highlight each compiler's unique features and template engines
- Ensure compatibility notes are clear and accurate
- Include appropriate Mermaid architecture diagrams
- Provide accurate installation and usage instructions

## Non-Goals (Out of Scope)
- Changing the actual functionality of the compilers
- Modifying the template structure beyond what's necessary for adaptation
- Creating new features or plugins

## Background & Context
- The compilers are Rust reimplementations of popular static site generators
- Each compiler has its own configuration format and unique features
- The existing READMEs are inconsistent and don't properly reflect each compiler's characteristics
- A template README has been created as a reference, but needs to be adapted for each compiler

## Functional Requirements
- **FR-1**: Each README must follow the general structure of the template
- **FR-2**: Configuration examples must use the compiler's actual configuration format
- **FR-3**: Unique features and template engines must be highlighted for each compiler
- **FR-4**: Installation and usage instructions must be accurate for each compiler
- **FR-5**: Mermaid architecture diagrams must be included
- **FR-6**: Compatibility notes must be clear and accurate

## Non-Functional Requirements
- **NFR-1**: READMEs must use lively, community-friendly language
- **NFR-2**: No absolute paths should be used
- **NFR-3**: READMEs must be in English
- **NFR-4**: Emojis should be used appropriately to enhance readability

## Constraints
- **Technical**: Must use the existing README template as a reference
- **Dependencies**: Must accurately reflect each compiler's actual features and configuration format

## Assumptions
- Each compiler has its own specific configuration format (TOML, YAML, JSON, JavaScript, etc.)
- Each compiler supports different template engines and features
- The Rust reimplementations maintain compatibility with the original compilers when using static features

## Acceptance Criteria

### AC-1: Consistent README Structure
- **Given**: All compiler READMEs
- **When**: Reviewed
- **Then**: They follow the same general structure as the template
- **Verification**: `human-judgment`

### AC-2: Accurate Configuration Examples
- **Given**: Each compiler's README
- **When**: Examined
- **Then**: Configuration examples use the compiler's actual format
- **Verification**: `human-judgment`

### AC-3: Unique Features Highlighted
- **Given**: Each compiler's README
- **When**: Reviewed
- **Then**: Unique features and template engines are clearly highlighted
- **Verification**: `human-judgment`

### AC-4: Accurate Installation Instructions
- **Given**: Each compiler's README
- **When**: Examined
- **Then**: Installation instructions use the correct compiler name
- **Verification**: `human-judgment`

### AC-5: Mermaid Architecture Diagrams
- **Given**: All compiler READMEs
- **When**: Reviewed
- **Then**: They include Mermaid architecture diagrams
- **Verification**: `human-judgment`

### AC-6: Clear Compatibility Notes
- **Given**: All compiler READMEs
- **When**: Examined
- **Then**: They include clear compatibility notes
- **Verification**: `human-judgment`

## Open Questions
- [ ] What are the specific configuration formats for each compiler?
- [ ] What are the unique features and template engines for each compiler?
- [ ] What are the specific installation commands for each compiler?
