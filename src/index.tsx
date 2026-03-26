import { createRoot } from "react-dom/client";
import App from "./App";
import "./index.css";

const node = document.getElementById("root") as HTMLElement;
const root = createRoot(node);

root.render(<App />);
