import CustomWalletButton from "./components/wallet-ui/CustomWalletButton";

function App() {
  return (
    <div className="min-h-screen">
      <header className="bg-white border-2 z-50 p-2 px-6 fixed top-0 w-screen sm:w-[calc(100vw-38px)] sm:left-[38px] flex justify-between">

        <section className="flex items-center gap-8">
          <h2 className="font-anton pl-2 text-3xl -skew-x-[24deg] justify-self-start">
            COCAINE.LEGAL
          </h2>

          <nav className="font-oxygen-mono">
            <ul className="flex gap-5">
              <li className="hover:underline">main</li>
              <li className="hover:underline">mint</li>
              <li className="hover:underline">forward</li>
              <li className="hover:underline">marketplace</li>
            </ul>
          </nav>
        </section>


        <div className="justify-self-end">
          <CustomWalletButton />
        </div>
      </header>

      <aside className="fixed z-50 top-14.5 sm:top-0">
        <InfiniteStripe />
      </aside>

      <main className="sm:ml-[38px]">
        <section className="pt-24 sm:pt-[60px] border-b-2 dotted-bg flex justify-center h-[950px]">
          <div className="flex bg-white h-fit mt-14 sm:mt-40 justify-center border-4 p-5 m-10 flex-col gap-2 w-[1000px] shadow-[8px_8px_0px_rgba(0,0,0,1)]">
            <h3 className="font-anton text-4xl sm:text-6xl lg:text-7xl xl:text-8xl break-all ">UUUUUUOOOHHHHHHHOOHOHHOOHHOOHOHHOHOHHOOHHOOHOHHOHOHOHOHHOOHOOHOHOHOHOHOHOOHOHHOOHHOHOHOHHOOHOHOHOOOO</h3>
            <p className="font-oxygen-mono">cringe edgy emails. ownership settled on solana. permissionless. magnificent.</p>
            <section className="-skew-x-12">
              <button className="border-2 font-oxygen-mono p-2">button to suck cock</button>
            </section>
          </div>
        </section>
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
    <aside className="border-2 h-5 sm:h-screen w-screen sm:w-10 bg-white overflow-hidden">
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
