# Prompt Engine Notes

## Philosophy Anchor

The internal reference prompt is constraint-first and anti-beauty:

- the world exists first
- the photograph happens afterwards
- the world does not cooperate
- the image should survive as a fragment of life, not as a successful artwork

## Design Direction

- Store prompt policies modularly, not as one giant string
- Keep country-aware context separate from global constraints
- Generate 8 frames with subtle bounded variation from one Roll DNA
- Treat alternate take as another nearby frame, not correction
- Treat people as incidental environmental fragments, not centered subjects or portrait candidates
- Push color and texture toward mundane automatic-camera output, avoiding generated-photo tells such as glossy surfaces, smooth AI skin, HDR clarity, cinematic grading, artificial grain, and designed palettes
- Keep obstructions incidental. Hands, sleeves, bags, straps, umbrellas, cups, and reflections may interrupt an edge or corner, but should not become centered substitute subjects.
- Recent external prompt guidance consistently points toward concrete capture conditions, mundane light, small fitting imperfections, off-center subjects, and avoiding vague quality/style boosters. For FoundFrame, do not import generic "photorealistic DSLR" or portrait-oriented prompt recipes directly; translate them into ordinary automatic-camera constraints instead.
- Camera and film references should stay behind the prompt engine. The app may use ordinary compact, disposable-camera, instant-camera, Lomo-like compact, or cheap point-and-shoot profiles, but the prompt should express their physical traits: soft edges, weak direct flash, lifted blacks, mild color cast, loose focus, or muted one-hour-print color.
- Avoid overcorrecting person-centered images into everyone-facing-away images. Incidental people can be front-facing, three-quarter, profile, looking down, partly blocked, small, or soft; variety matters more than hiding every face.
- Local `_sample/` references showed that usable naturalness often comes from photographed surfaces and everyday public signs rather than from human subjects: shopfronts, signs, shelves, vending machines, chairs, walls, windows, road markings, tables, price boards, packaging, numbers, and plain empty space. Prompt changes should preserve this as an abstraction only; do not copy or commit the reference images.
- Let some frames be place-led or object-led. Avoid forcing every frame to solve around a person, face, hand, or single central object.
- Treat `Tiny Detail` as an optional clue, not a foreground prop contract. It may appear in one or two frames, move to an edge/background/surface trace, or be absent. Never repeat one cup, tumbler, bag, hand, tray, or bottle across every contact-sheet frame.

## Review Implications

The review engine should score for:

- low intentionality
- everyday-life density
- accidental feeling
- memory quality
- imperfection
- non-heroic composition
