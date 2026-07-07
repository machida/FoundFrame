# Kickoff Spec Review

## Overall Assessment

The product concept is coherent and differentiated. The strongest part of the spec is its philosophical clarity: the app simulates ordinary photography and accidental memory rather than image authorship.

## What Is Strong

- Clear product philosophy
- Hidden prompt requirement
- Good distinction between country and style
- SQLite-backed dictionary direction
- Extensibility-first mindset
- Contact sheet and frame selection workflow

## Improvements Recommended

- Separate situation input, Roll DNA, prompt assembly, generation, review, and archive layers
- Treat Roll DNA as a versioned contract
- Keep dictionary data separate from prompt semantics
- Add provider abstraction early
- Make evaluation pluggable and asynchronous
- Define failure states explicitly
- Add editorial governance to avoid tourist clichés and country stereotyping

## Key Risks

- Models may drift toward cinematic or idealized imagery
- Random may feel arbitrary without coherent world logic
- Users may optimize for prettiness unless UI steers them
- Country dictionaries may drift into stereotype packs
- Hidden prompt logic can make debugging difficult without observability
- Reproducibility breaks if provider, prompt-engine, and dictionary versions are not persisted together

## Current Clarified Decisions

- Provider: OpenAI first
- Contact sheet: 8 individual frame generations shown as one sheet
- Review Engine v1: rule-based
- Dictionary authoring: YAML source files imported into SQLite
- Setup defaults: unspecified variables are app-randomized unless the user explicitly fixes them
- UX direction: primary UI should describe photographic process, not prompt/provider internals
- Fallback mode: when remote generation is unavailable, local study mode still exercises the workflow
