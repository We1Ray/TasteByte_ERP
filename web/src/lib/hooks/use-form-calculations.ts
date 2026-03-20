"use client";

import { useCallback } from "react";

interface Formula {
  target_field: string;
  formula: string;
  trigger_fields: string[];
}

function evaluateFormula(formula: string, data: Record<string, unknown>): number | null {
  try {
    return evalExpression(formula.trim(), data);
  } catch {
    return null;
  }
}

// Two-pass: handle +/- first, then */ within terms
function evalExpression(expr: string, data: Record<string, unknown>): number | null {
  const terms: number[] = [];
  const ops: string[] = [];
  let current = '';

  for (const ch of expr) {
    if ((ch === '+' || ch === '-') && current.trim().length > 0) {
      const val = evalTerm(current.trim(), data);
      if (val === null) return null;
      terms.push(val);
      ops.push(ch);
      current = '';
    } else {
      current += ch;
    }
  }
  if (current.trim()) {
    const val = evalTerm(current.trim(), data);
    if (val === null) return null;
    terms.push(val);
  }

  if (terms.length === 0) return null;
  let result = terms[0];
  for (let i = 0; i < ops.length; i++) {
    if (ops[i] === '+') result += terms[i + 1];
    else if (ops[i] === '-') result -= terms[i + 1];
  }
  return Math.round(result * 100) / 100;
}

function evalTerm(term: string, data: Record<string, unknown>): number | null {
  const factors: number[] = [];
  const ops: string[] = [];
  let current = '';

  for (const ch of term) {
    if ((ch === '*' || ch === '/') && current.trim().length > 0) {
      const val = resolveValue(current.trim(), data);
      if (val === null) return null;
      factors.push(val);
      ops.push(ch);
      current = '';
    } else {
      current += ch;
    }
  }
  if (current.trim()) {
    const val = resolveValue(current.trim(), data);
    if (val === null) return null;
    factors.push(val);
  }

  if (factors.length === 0) return null;
  let result = factors[0];
  for (let i = 0; i < ops.length; i++) {
    if (ops[i] === '*') result *= factors[i + 1];
    else if (ops[i] === '/') {
      if (factors[i + 1] === 0) return null;
      result /= factors[i + 1];
    }
  }
  return result;
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
