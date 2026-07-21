import Main from "./components/pages/Main";
import CustomWalletButton from "./components/wallet-ui/CustomWalletButton";

function App() {
  return (
    <div className="min-h-screen">
      <header className="bg-white border-2 z-50 p-2 px-6 fixed top-0 w-screen flex items-center justify-between">
        <section className="flex items-center gap-8">
          <h2 className="font-anton pl-2 text-3xl -skew-x-[24deg] justify-self-start">
            COCAINE.LEGAL
          </h2>

          <nav className="font-oxygen-mono h-full flex items-center">
            <ul className="flex h-full items-center">
              <li className="p-3 cursor-pointer hover:-skew-x-12 hover:underline transition-all duration-75">main</li>
              <li className="p-3 cursor-pointer hover:-skew-x-12 hover:underline transition-all duration-75">mint</li>
              <li className="p-3 cursor-pointer hover:-skew-x-12 hover:underline transition-all duration-75">forward</li>
              <li className="p-3 cursor-pointer hover:-skew-x-12 hover:underline transition-all duration-75">marketplace</li>
            </ul>
          </nav>
        </section>


        <div className="justify-self-end">
          <CustomWalletButton />
        </div>
      </header>

      <main className="border-l-2 border-r-2">
        <Main/>
      </main>
    </div>
  );
}

export default App;
