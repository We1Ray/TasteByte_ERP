import { describe, it, expect, beforeEach } from "vitest";
import { useBuilderStore } from "@/lib/stores/builder-store";
import type { FormSection, FieldDefinition } from "@/lib/types/lowcode";

function makeSection(id: string, fields: FieldDefinition[] = []): FormSection {
  return {
    id,
    title: `Section ${id}`,
    columns: 1,
    collapsible: false,
    collapsed_default: false,
    sort_order: 0,
    fields,
  };
}

function makeField(id: string, sectionId: string): FieldDefinition {
  return {
    id,
    section_id: sectionId,
    field_key: `field_${id}`,
    field_type: "text",
    label: `Field ${id}`,
    validation: {},
    sort_order: 0,
  };
}

describe("builder-store", () => {
  beforeEach(() => {
    // Reset store state before each test
    useBuilderStore.setState({
      sections: [],
      selectedFieldId: null,
      selectedSectionId: null,
      isDirty: false,
    });
  });

  it("addSection sets dirty flag", () => {
    const store = useBuilderStore.getState();
    expect(store.isDirty).toBe(false);

    store.addSection(makeSection("s1"));

    const updated = useBuilderStore.getState();
    expect(updated.sections).toHaveLength(1);
    expect(updated.sections[0].id).toBe("s1");
    expect(updated.isDirty).toBe(true);
  });

  it("deleteSection clears selection when deleted section was selected", () => {
    const store = useBuilderStore.getState();
    store.addSection(makeSection("s1"));
    store.addSection(makeSection("s2"));
    store.selectSection("s1");

    expect(useBuilderStore.getState().selectedSectionId).toBe("s1");

    useBuilderStore.getState().deleteSection("s1");

    const updated = useBuilderStore.getState();
    expect(updated.sections).toHaveLength(1);
    expect(updated.selectedSectionId).toBeNull();
    expect(updated.selectedFieldId).toBeNull();
  });

  it("addField adds field to the correct section", () => {
    const store = useBuilderStore.getState();
    store.addSection(makeSection("s1"));
    store.addSection(makeSection("s2"));

    store.addField("s2", makeField("f1", "s2"));

    const updated = useBuilderStore.getState();
    expect(updated.sections[0].fields).toHaveLength(0);
    expect(updated.sections[1].fields).toHaveLength(1);
    expect(updated.sections[1].fields[0].id).toBe("f1");
    expect(updated.isDirty).toBe(true);
  });

  it("reorderSections swaps section positions and updates sort_order", () => {
    const store = useBuilderStore.getState();
    store.addSection(makeSection("s1"));
    store.addSection(makeSection("s2"));
    store.addSection(makeSection("s3"));

    useBuilderStore.getState().reorderSections("s3", "s1");

    const updated = useBuilderStore.getState();
    expect(updated.sections.map((s) => s.id)).toEqual(["s3", "s1", "s2"]);
    expect(updated.sections[0].sort_order).toBe(0);
    expect(updated.sections[1].sort_order).toBe(1);
    expect(updated.sections[2].sort_order).toBe(2);
    expect(updated.isDirty).toBe(true);
  });

  it("markClean resets dirty flag", () => {
    const store = useBuilderStore.getState();
    store.addSection(makeSection("s1"));
    expect(useBuilderStore.getState().isDirty).toBe(true);

    useBuilderStore.getState().markClean();
    expect(useBuilderStore.getState().isDirty).toBe(false);
  });
});
