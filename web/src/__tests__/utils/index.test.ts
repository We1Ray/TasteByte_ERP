import { describe, it, expect } from "vitest";
import { cn, formatCurrency, formatNumber, statusColor, formatDate, formatDateTime } from "@/lib/utils";

describe("cn", () => {
  it("merges class names", () => {
    expect(cn("foo", "bar")).toBe("foo bar");
  });

  it("handles conditional classes", () => {
    expect(cn("base", false && "hidden", "visible")).toBe("base visible");
  });
});

describe("formatCurrency", () => {
  it("formats USD by default with two decimal places", () => {
    expect(formatCurrency(1234.5)).toBe("$1,234.50");
  });

  it("formats zero correctly", () => {
    expect(formatCurrency(0)).toBe("$0.00");
  });

  it("formats negative amounts", () => {
    expect(formatCurrency(-500)).toBe("-$500.00");
  });
});

describe("formatNumber", () => {
  it("formats integer with no decimals by default", () => {
    expect(formatNumber(12345)).toBe("12,345");
  });

  it("formats with specified decimal places", () => {
    expect(formatNumber(12345.678, 2)).toBe("12,345.68");
  });

  it("formats zero", () => {
    expect(formatNumber(0)).toBe("0");
  });
});

describe("statusColor", () => {
  it("returns green for completed status", () => {
    expect(statusColor("completed")).toBe("green");
  });

  it("returns gray for unknown status", () => {
    expect(statusColor("unknown_xyz")).toBe("gray");
  });

  it("is case-insensitive", () => {
    expect(statusColor("DRAFT")).toBe("gray");
    expect(statusColor("Active")).toBe("blue");
  });

  it("returns amber for in-progress statuses", () => {
    expect(statusColor("in_progress")).toBe("amber");
    expect(statusColor("in progress")).toBe("amber");
    expect(statusColor("pending")).toBe("amber");
  });

  it("returns blue for open/released statuses", () => {
    expect(statusColor("released")).toBe("blue");
    expect(statusColor("open")).toBe("blue");
  });

  it("returns red for closed/cancelled/rejected statuses", () => {
    expect(statusColor("closed")).toBe("red");
    expect(statusColor("cancelled")).toBe("red");
    expect(statusColor("rejected")).toBe("red");
  });

  it("returns green for done/approved statuses", () => {
    expect(statusColor("done")).toBe("green");
    expect(statusColor("approved")).toBe("green");
  });
});

describe("formatDate", () => {
  it("formats a date string", () => {
    const result = formatDate("2024-01-15T00:00:00Z");
    expect(result).toContain("Jan");
    expect(result).toContain("15");
    expect(result).toContain("2024");
  });

  it("formats a Date object", () => {
    const result = formatDate(new Date("2024-06-20T00:00:00Z"));
    expect(result).toContain("Jun");
    expect(result).toContain("20");
    expect(result).toContain("2024");
  });
});

describe("formatDateTime", () => {
  it("formats a date string with time", () => {
    const result = formatDateTime("2024-01-15T14:30:00Z");
    expect(result).toContain("Jan");
    expect(result).toContain("15");
    expect(result).toContain("2024");
  });

  it("formats a Date object with time", () => {
    const result = formatDateTime(new Date("2024-06-20T09:15:00Z"));
    expect(result).toContain("Jun");
    expect(result).toContain("20");
    expect(result).toContain("2024");
  });
});
