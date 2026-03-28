import ReactDiffViewer, { DiffMethod } from "react-diff-viewer";
import hljs from "highlight.js";
import "highlight.js/styles/atom-one-light.css";

interface DiffViewerProps {
  input: string;
  output: string;
}

const renderContent = (text: string) => {
  try {
    const highlighted = hljs.highlight(text, { language: "bash", ignoreIllegals: true }).value;
    return (
      <pre
        style={{ margin: 0, whiteSpace: "pre-wrap", wordBreak: "break-word" }}
        dangerouslySetInnerHTML={{ __html: highlighted }}
      />
    );
  } catch {
    return <pre style={{ margin: 0, whiteSpace: "pre-wrap", wordBreak: "break-word" }}>{text}</pre>;
  }
};

const diffStyles = {
  line: { paddingLeft: "8px", paddingRight: "8px" },
  gutter: { backgroundColor: "#f5f5f5", minWidth: "50px" },
  marker: { width: "30px", display: "inline-block", paddingLeft: "5px" },
  variables: {
    light: {
      diffViewerBackground: "#ffffff",
      diffViewerColor: "#212529",
      addedBackground: "#e8f5e9",
      addedColor: "#1b5e20",
      removedBackground: "#ffebee",
      removedColor: "#b71c1c",
      wordAddedBackground: "#a6e22e",
      wordRemovedBackground: "#ff6b6b",
      addedGutterBackground: "#e8f5e9",
      removedGutterBackground: "#ffebee",
      gutterBackground: "#f8f9fa",
      gutterBorderColor: "#e9ecef",
      codeFoldGutterBackground: "#dbedff",
      codeFoldBackground: "#f1f8ff",
      emptyLineBackground: "#f5f5f5",
      lineNumberColor: "#959da5",
      lineNumberBackground: "#f8f9fa",
    },
  },
};

export default function DiffViewer({ input, output }: DiffViewerProps) {
  return (
    <div>
      <h3 className="text-lg font-semibold text-text-dark mt-8 mb-4">Git patch</h3>
      <div className="rounded-lg overflow-hidden bg-white min-h-96">
        <ReactDiffViewer
          oldValue={input}
          newValue={output}
          splitView
          compareMethod={DiffMethod.WORDS}
          showDiffOnly={false}
          renderContent={renderContent}
          codeFoldMessageRenderer={() => null}
          styles={diffStyles}
        />
      </div>
    </div>
  );
}
