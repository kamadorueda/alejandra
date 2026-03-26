import { useMemo } from "react";
import ReactDiffViewer, { DiffMethod } from "react-diff-viewer";
import "react-diff-viewer/bundles/react-diff-viewer.css";

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
    }),
    [input, output]
  );

  return (
    <div className="border border-neutral-200 rounded-lg overflow-hidden bg-white">
      <ReactDiffViewer {...diffConfig} />
    </div>
  );
}
