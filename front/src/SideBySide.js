import react from "react";
import ReactDiffViewer from "react-diff-viewer";
import * as wasm from "alejandra-front";
import { Editor } from "./Editor";

export const SideBySide = () => {
  const [loading, setLoading] = react.useState(true);
  const [before, setBefore] = react.useState("let x = 123; in\n x");

  react.useEffect(async () => {
    await wasm.default();
    setLoading(false);
  }, [wasm]);

  if (loading) {
    return <span>Loading</span>;
  }

  const after = wasm.format(before, "before.nix");

  return (
    <react.Fragment>
      <div className="flex items-center justify-center">
        <div className="f6 fl tc w-40">Type your code below</div>
        <div className="w-10" />
        <div className="f6 fl tc w-40">With Alejandra ❤️</div>
      </div>
      <div className="flex items-center justify-center pt1">
        <div className="f6 fl w-40">
          <Editor code={before} onChange={setBefore} />
        </div>
        <div className="w-10" />
        <div className="f6 fl w-40">
          <Editor code={after} onChange={() => {}} />
        </div>
      </div>
      <div className="flex items-center justify-center pt4">
        <div className="f6 fl tc w-80">Git patch</div>
      </div>
      <div className="flex items-center justify-center pt1">
        <div className="f6 fl w-80">
          <ReactDiffViewer oldValue={before} newValue={after} />
        </div>
      </div>
    </react.Fragment>
  );
};
