import Main from "./components/pages/Main";
import CustomWalletButton from "./components/wallet-ui/CustomWalletButton";

function App() {
  return (
    <div className="min-h-screen">
      <header className="bg-white border-2 z-50 p-2 px-6 fixed top-0 w-screen sm:w-[calc(100vw-38px)] sm:left-[38px] flex items-center justify-between">
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

      <aside className="fixed z-30 top-16 sm:top-0">
        <InfiniteStripe />
      </aside>

      <main className="sm:ml-[38px]">
        <Main/>
      </main>
    </div>
  );
}

export default App;

const STRIPE_TEXT = "resist cocaine";
const REPEAT_COUNT = 15;

function InfiniteStripe() {
  const items = Array.from({ length: REPEAT_COUNT });
  return (
    <aside className="border-2 p-1 sm:h-screen w-screen sm:w-10 bg-white overflow-hidden">
      <div className="flex sm:flex-col animate-horizontal sm:animate-vertical will-change-transform">
        <div className="flex sm:flex-col items-center gap-2 shrink-0">
          {items.map((_, i) => (
            <span
              key={`a-${i}`}
              className="sm:[writing-mode:vertical-rl] text-sm font-oxygen-mono italic shrink-0"
            >
              {STRIPE_TEXT}
            </span>
          ))}
        </div>
      </div>
    </aside>
  );
}
