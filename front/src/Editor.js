import "codemirror/lib/codemirror.css";
import "codemirror/mode/javascript/javascript";
import "codemirror/theme/monokai.css";
import { Controlled as CodeMirror } from "react-codemirror2";

export const Editor = ({ code, onChange }) => {
  return (
    <CodeMirror
      value={code}
      options={{
        mode: "javascript",
        theme: "monokai",
        lineNumbers: true,
      }}
      onBeforeChange={(_, __, value) => {
        onChange(value);
      }}
    />
  );
};
