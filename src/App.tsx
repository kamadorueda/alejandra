import SideBySide from "./components/SideBySide";

export default function App() {
  return (
    <div className="min-h-screen bg-surface py-8">
      <div className="mx-auto max-w-4xl px-4">
        <h1 className="text-center text-4xl font-bold text-text-dark">Alejandra 💅</h1>
        <p className="mt-2 text-center text-sm text-text">
          The Uncompromising Nix Code Formatter
        </p>
        <hr className="my-6 border-neutral-200" />
        <SideBySide />
      </div>
    </div>
  );
}
