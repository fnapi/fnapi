import { it, describe, expect } from "@jest/globals";
import { provide } from "../context";

describe("context", () => {
  it("should work", () => {
    const MyProvider = provide(async (req, res) => {
      return "foo";
    });

    expect(MyProvider).toBeTruthy();
  });
});
