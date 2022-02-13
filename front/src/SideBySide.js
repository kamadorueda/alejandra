import react from "react";
import ReactDiffViewer from "react-diff-viewer";
import * as wasm from "alejandra-front";
import { Editor } from "./Editor";
import { get, randomPath } from "./nixpkgs";

const getPermalink = (before, path) => {
  const searchParams = new URLSearchParams();

  if (path !== undefined) {
    searchParams.append("path", path);
  } else {
    searchParams.append("before", before);
  }

  return (
    window.location.origin +
    window.location.pathname +
    `?${searchParams.toString()}`
  );
};

export const SideBySide = () => {
  const [path, setPath] = react.useState(undefined);
  const [loading, setLoading] = react.useState(true);
  const [before, setBefore] = react.useState("");

  react.useEffect(async () => {
    const searchParams = new URLSearchParams(window.location.search);

    if (searchParams.has("path")) {
      setPath(searchParams.get("path"));
    } else if (searchParams.has("before")) {
      setBefore(searchParams.get("before"));
    } else {
      setPath(randomPath());
    }
  }, []);

  react.useEffect(async () => {
    await wasm.default();
    setLoading(false);
  }, [wasm]);

  react.useEffect(async () => {
    if (path !== undefined) {
      const content = await get(path);
      setBefore(content);
    }
  }, [path]);

  if (loading) {
    return (
      <div className="flex items-center justify-center">
        <div className="f6 tc">Loading</div>
      </div>
    );
  }

  const after = wasm.format(before, "before.nix");

  const permalink = getPermalink(before, path);

  return (
    <react.Fragment>
      <div className="flex items-center justify-center">
        <div className="f6 fl tc w-40">
          Type your code below or&nbsp;
          <span
            className="blue underline"
            onClick={() => {
              setPath(randomPath());
            }}
          >
            click here to fetch a random file from Nixpkgs
          </span>
        </div>
        <div className="w-10" />
        <div className="f6 fl tc w-40">With Alejandra ❤️</div>
      </div>
      <div className="flex items-center justify-center pt1">
        <div className="f6 fl w-40">
          <Editor
            code={before}
            onChange={(code) => {
              setBefore(code);
              setPath(undefined);
            }}
          />
          <div className="flex items-center justify-center">
            <span>Permalink:&nbsp;</span>
            {permalink.length > 2048 ? (
              "not available, exceeds 2048 characters"
            ) : (
              <a className="blue underline" href={permalink}>
                here
              </a>
            )}
          </div>
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
          <ReactDiffViewer
            disableWordDiff={true}
            newValue={after}
            oldValue={before}
          />
        </div>
      </div>
    </react.Fragment>
  );
};
