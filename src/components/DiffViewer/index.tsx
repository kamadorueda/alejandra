import { useMemo } from "react";
import ReactDiffViewer, { DiffMethod } from "react-diff-viewer";

interface DiffViewerProps {
  input: string;
  output: string;
}

export default function DiffViewer({ input, output }: DiffViewerProps) {
  // Memoize the diff calculation to avoid unnecessary re-renders
  const diffConfig = useMemo(
    () => ({
      oldValue: input,
      newValue: output,
      splitView: true,
      compareMethod: DiffMethod.WORDS,
      showDiffOnly: false,
      renderContent: (text: string) => {
        return <pre style={{ margin: 0, whiteSpace: "pre-wrap", wordBreak: "break-word" }}>{text}</pre>;
      },
      codeFoldMessageRenderer: () => null,
      styles: {
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
      },
    }),
    [input, output]
  );

  return (
    <div>
      <h3 className="text-lg font-semibold text-text-dark mt-8 mb-4">Git patch</h3>
      <div className="rounded-lg overflow-hidden bg-white min-h-96">
        <ReactDiffViewer {...diffConfig} />
      </div>
    </div>
  );
}
