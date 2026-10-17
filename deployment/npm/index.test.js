// @ts-check
const assert = require("assert");
const createFromBuffer = require("@dprint/formatter").createFromBuffer;
const getBuffer = require("./index").getBuffer;

const formatter = createFromBuffer(getBuffer());
const result = formatter.formatText({
  filePath: "file.cedar",
  fileText: "permit(principal,action,resource);",
});

assert.strictEqual(result, "permit (principal, action, resource);\n");
