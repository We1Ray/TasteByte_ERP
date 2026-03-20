"use client";

import { useCallback } from "react";

interface Formula {
  target_field: string;
  formula: string;
  trigger_fields: string[];
}

function evaluateFormula(formula: string, data: Record<string, unknown>): number | null {
  try {
    // Simple expression evaluator supporting: field references, numbers, +, -, *, /
    const tokens = formula.split(/([+\-*/])/).map(t => t.trim()).filter(Boolean);

    if (tokens.length === 1) {
      // Single value
      return resolveValue(tokens[0], data);
    }

    // Evaluate left to right (respecting * and / precedence would be more complex)
    // For now, simple left-to-right evaluation
    let result = resolveValue(tokens[0], data);
    if (result === null) return null;

    for (let i = 1; i < tokens.length; i += 2) {
      const op = tokens[i];
      const right = resolveValue(tokens[i + 1], data);
      if (right === null) return null;

      switch (op) {
        case '+': result += right; break;
        case '-': result -= right; break;
        case '*': result *= right; break;
        case '/': result = right !== 0 ? result / right : null; break;
      }
      if (result === null) return null;
    }

    return Math.round(result * 100) / 100; // Round to 2 decimal places
  } catch {
    return null;
  }
}

function resolveValue(token: string, data: Record<string, unknown>): number | null {
  const trimmed = token.trim();
  // Try as number
  const num = Number(trimmed);
  if (!isNaN(num)) return num;
  // Try as field reference
  const val = data[trimmed];
  if (typeof val === 'number') return val;
  if (typeof val === 'string') {
    const parsed = Number(val);
    if (!isNaN(parsed)) return parsed;
  }
  return null;
}

export function useFormCalculations(formulas: Formula[]) {
  const calculate = useCallback(
    (fieldName: string, currentData: Record<string, unknown>): Record<string, unknown> => {
      const updates: Record<string, unknown> = {};

      for (const formula of formulas) {
        if (formula.trigger_fields.includes(fieldName)) {
          const result = evaluateFormula(formula.formula, currentData);
          if (result !== null) {
            updates[formula.target_field] = result;
          }
        }
      }

      return updates;
    },
    [formulas]
  );

  return { calculate };
}
