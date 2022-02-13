import react from "react";
import { SideBySide } from "./SideBySide";
import "tachyons";

const App = () => {
  return (
    <react.Fragment>
      <h1 className="f2 tc">Alejandra 💅</h1>
      <h2 className="f6 normal tc">The Uncompromising Nix Code Formatter</h2>
      <br />
      <hr className="w-80" />
      <br />
      <SideBySide />
    </react.Fragment>
  );
};

export default App;
