# @dprint/cedar

npm distribution of [dprint-plugin-cedar](https://github.com/dprint/dprint-plugin-cedar) which is an adapter plugin for [cedar-policy-formatter](https://github.com/cedar-policy/cedar).

Use this with [@dprint/formatter](https://github.com/dprint/js-formatter) or just use @dprint/formatter and download the [dprint-plugin-cedar Wasm file](https://github.com/dprint/dprint-plugin-cedar/releases).

## Example

```ts
import { getBuffer } from "@dprint/cedar";
import { createFromBuffer } from "@dprint/formatter";

const formatter = createFromBuffer(getBuffer());

console.log(formatter.formatText({
  filePath: "policy.cedar",
  fileText: "permit(principal,action,resource);",
}));
```
