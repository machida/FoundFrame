import { describe, expect, it } from "vitest";
import { setupSituationFeel } from "./setupPresentation";

describe("setupSituationFeel", () => {
  it("uses Japan-specific transit wording for shared-route situations", () => {
    const result = setupSituationFeel({
      countryCode: "jp",
      moment: "leaving the station without being in a hurry",
      place: "train station underpass with ticket machines off to the side",
      time: "evening",
      season: "autumn",
      weather: "cloudy",
      tinyDetail: "convenience store receipt folded into a pocket",
    });

    expect(result).toContain("shared edges and short pauses");
    expect(result).toContain("route people know by repetition more than by memory");
    expect(result).toContain("practical routines tightening back up");
  });

  it("uses United States-specific car-threshold wording for parking-lot situations", () => {
    const result = setupSituationFeel({
      countryCode: "us",
      moment: "heading across the parking lot after a short store run",
      place: "neighborhood gas station side lot",
      time: "late_afternoon",
      season: "summer",
      weather: "humid",
      tinyDetail: "warm deli container fogging a thin plastic bag",
    });

    expect(result).toContain("sidewalks, parking lots, and car-adjacent pauses");
    expect(result).toContain("space between arriving and actually getting out");
    expect(result).toContain("day carrying a little extra weight");
  });

  it("falls back to generic place wording when no country-specific branch matches", () => {
    const result = setupSituationFeel({
      countryCode: "us",
      moment: "lingering by the laundry machines until one opens up",
      place: "plain corner near a public walkway",
      time: "morning",
      season: "spring",
      weather: "clear",
      tinyDetail: "loose laundry sheet sticking out of a basket",
    });

    expect(result).toContain("ordinary movement stretched across sidewalks, parking lots, and car-adjacent pauses");
    expect(result).toContain("day is still settling in");
    expect(result).toContain("routines have gone slightly loose at the edges");
    expect(result).toContain("passes through more often than they notice");
  });

  it("provides a Japanese situation reading", () => {
    const result = setupSituationFeel({
      countryCode: "jp",
      moment: "leaving the station without being in a hurry",
      place: "train station underpass",
      time: "evening",
      season: "autumn",
      weather: "cloudy",
      tinyDetail: "a folded receipt",
    }, "ja");

    expect(result).toContain("日本の日常");
    expect(result).toContain("夕方");
    expect(result).toContain("曇り");
    expect(result).toContain("秋らしさ");
  });
});
