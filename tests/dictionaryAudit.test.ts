import { execFileSync } from "node:child_process";
import { resolve } from "node:path";
import { describe, expect, it } from "vitest";

describe("dictionary coverage audit script", () => {
  it("prints both countries and the expected metric sections", () => {
    const scriptPath = resolve(process.cwd(), "scripts/dictionary/audit-coverage.sh");
    const output = execFileSync("bash", [scriptPath], {
      cwd: process.cwd(),
      encoding: "utf-8",
    });

    expect(output).toContain("FoundFrame dictionary coverage audit");
    expect(output).toContain("[jp]");
    expect(output).toContain("[us]");
    expect(output).toContain("- moments.yaml");
    expect(output).toContain("- places.yaml");
    expect(output).toContain("- object-details.yaml");
    expect(output).toContain("time_context:");
    expect(output).toContain("weather:");
    expect(output).toContain("seasonality:");
    expect(output).toContain("Heuristic only:");
  });

  it("reports non-zero entry counts for current country files", () => {
    const scriptPath = resolve(process.cwd(), "scripts/dictionary/audit-coverage.sh");
    const output = execFileSync("bash", [scriptPath], {
      cwd: process.cwd(),
      encoding: "utf-8",
    });

    const entryLines = output
      .split("\n")
      .filter((line) => line.trimStart().startsWith("entries:"));

    expect(entryLines.length).toBeGreaterThanOrEqual(6);
    for (const line of entryLines) {
      const match = line.match(/entries:\s+(\d+)/);
      expect(match).not.toBeNull();
      expect(Number(match?.[1] ?? 0)).toBeGreaterThan(0);
    }
  });
});
