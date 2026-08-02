/** @owned */
const state = createState();
const writer = borrowMut(state);
const reader = borrow(state);
writer.set("ready", true);
reader.get("ready");
