import react from "react";
import * as wasm from "alejandra-front";
import { Editor } from "./Editor";

export const SideBySide = () => {
  const [loading, setLoading] = react.useState(true);
  const [before, setBefore] = react.useState("let x = 123; in x");

  react.useEffect(async () => {
    await wasm.default();
    setLoading(false);
  }, [wasm]);

  if (loading) {
    return <span>Loading</span>;
  }

  return (
    <react.Fragment>
      <div className="flex items-center justify-center">
        <div className="fl tc w-40">Type your code below</div>
        <div className="w-10" />
        <div className="fl tc w-40">With Alejandra ❤️</div>
      </div>
      <br />
      <div className="flex items-center justify-center">
        <div className="fl h5 w-40">
          <Editor code={before} onChange={setBefore} />
        </div>
        <div className="w-10" />
        <div className="fl h5 w-40">
          <Editor
            code={wasm.format(before, "before.nix")}
            onChange={() => {}}
          />
        </div>
      </div>
    </react.Fragment>
  );
};
