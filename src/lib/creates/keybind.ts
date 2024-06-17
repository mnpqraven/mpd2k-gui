import { z } from "zod";

const mod = z.enum(["ctrl", "alt", "shift"]);
const createArg = z.object({
  key: z.string().length(1),
  modifiers: z.array(mod),
  action: z.function().args(z.custom<KeyboardEvent>()).returns(z.void()),
});
type CreateArg = z.infer<typeof createArg>;

export function createKeybind(_opt: CreateArg) {
  const { modifiers, key, action } = createArg.parse(_opt);

  function handleKeyDown(e: KeyboardEvent) {
    const modbool = modifiers.map((f) => getModTruth(f)(e)).every((f) => f);

    if (e.key === key && modbool) {
      e.preventDefault();
      action(e);
    }
  }

  return { handleKeyDown };
}

function getModTruth(val: z.infer<typeof mod>): (e: KeyboardEvent) => boolean {
  switch (val) {
    case "ctrl":
      return (e) => e.ctrlKey || e.metaKey;
    case "alt":
      return (e) => e.altKey;
    case "shift":
      return (e) => e.shiftKey;
  }
}
