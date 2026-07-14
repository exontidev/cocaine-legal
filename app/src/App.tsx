import CustomWalletButton from "./components/wallet-ui/CustomWalletButton";

function App() {
  return (
    <div className="min-h-screen">
      <header className="bg-white border-2 p-2 px-6 fixed top-0 w-[calc(100vw-38px)] left-[38px] grid grid-cols-[1fr_auto] items-center z-10">
        <h2 className="font-anton pl-2 text-3xl -skew-x-[24deg] justify-self-start">
          COCAINE.LEGAL
        </h2>
        <div className="justify-self-end">
          <CustomWalletButton />
        </div>
      </header>

      <aside className="fixed top-0 left-0 h-screen w-[38px]">
        <InfiniteStripe />
      </aside>

      <main className="ml-[38px] pt-[64px]">
        <section className="border-b-2 dotted-bg flex justify-center h-[900px]">
          <div className="flex bg-white h-fit mt-40 justify-center border-4 p-5 m-10 flex-col gap-2 skew-y-12 w-[1000px] shadow-[8px_8px_0px_rgba(0,0,0,1)]">
            <h3 className="font-anton text-4xl sm:text-8xl break-all ">UUUUUUOOOHHHHHHHOOHOHHOOHHOOHOHHOHOHHOOHHOOHOHHOHOHOHOHHOOHOOHOHOHOHOHOHOOHOHHOOHHOHOHOHHOOHOHOHOOOO</h3>
            <p className="font-oxygen-mono">cringe edgy emails. ownership settled on solana. unstoppable. magnificent.</p>
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
    <aside className="border-2 h-screen w-10 bg-white overflow-hidden">
      <div className="flex flex-col items-center gap-2 animate-marquee">
        {items.map((_, i) => (
          <span
            key={`a-${i}`}
            className="[writing-mode:vertical-rl] text-sm font-oxygen-mono italic shrink-0"
          >
            {STRIPE_TEXT}
          </span>
        ))}
        {items.map((_, i) => (
          <span
            key={`b-${i}`}
            className="[writing-mode:vertical-rl] text-sm font-oxygen-mono italic shrink-0"
          >
            {STRIPE_TEXT}
          </span>
        ))}
      </div>

      <style>{`
        @keyframes marquee-scroll {
          from { transform: translateY(-50%); }
          to   { transform: translateY(0); }
        }
        .animate-marquee {
          animation: marquee-scroll 20s linear infinite;
        }
      `}</style>
    </aside>
  );
}
